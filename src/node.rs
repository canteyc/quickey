use crate::{
    keyboard::{self, distances, sorted_distances},
    math::{cmp_f32, frac},
};
use serde::{
    Deserialize, Serialize, Serializer,
    de::{Error, MapAccess, Visitor},
    ser::SerializeStruct,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn predict(root: &Node, points: Vec<(f32, f32)>) -> Vec<String> {
    let mut results = vec![];
    let mut node = root;
    let mut buf = String::new();
    for (x, y) in points {
        let dist = distances(&keyboard::ALPHA, x, y);
        let probs = dist.iter().filter_map(|(k, d)| {
            let p = node.children().find_map(|child| {
                if child.c.eq(&k.ch) {
                    let average_next_usage = frac(child.next_usage, child.next_count) as u32;
                    Some((
                        frac(child.usage as u32, average_next_usage) as f32 * d,
                        child,
                    ))
                } else {
                    None
                }
            });
            p.map(|p| (k, p.0, p.1))
        });
        dbg!(&probs);
        if let Some((key, _, child)) = probs.max_by(|a, b| cmp_f32(b.1, a.1)) {
            buf.push(char::from_u32(key.ch as u32).unwrap());
            node = child;
        }
    }
    results.push(buf);
    results
}

pub struct Node {
    pub c: u8,
    usage: u16,
    next_count: u32,
    next_usage: u32,
    next: Vec<Node>,
}

pub struct SortedSliceIterator<'a, T: PartialOrd> {
    slice: &'a [T],
    order: Vec<usize>,
    current: usize,
}

impl Node {
    pub fn new(c: u8) -> Self {
        Self {
            c,
            usage: 1,
            next_count: 0,
            next_usage: 0,
            next: Default::default(),
        }
    }

    pub fn load_dictionary_from_txt(dict_file: &Path) -> Node {
        let mut dict_reader = BufReader::new(File::open(dict_file).unwrap());
        let mut buf = String::with_capacity(32);
        let mut root = Node::new(0u8);
        while dict_reader.read_line(&mut buf).is_ok_and(|s| s > 0) {
            root.add_usages(&buf, 1);
            buf.clear();
        }
        root
    }

    pub fn load_dictionary_from_json(dict_json: &Path) -> Node {
        serde_json::from_reader(BufReader::new(File::open(dict_json).unwrap())).unwrap()
    }

    pub fn load_dictionary_from_csv(dict_csv: &Path) -> Node {
        let mut dict_reader = BufReader::new(File::open(dict_csv).unwrap());
        let mut buf = String::with_capacity(64);
        dict_reader.read_line(&mut buf).unwrap();
        buf.clear();
        let mut root = Node::new(0u8);
        while dict_reader.read_line(&mut buf).is_ok_and(|s| s > 0) {
            let (word, usage) = buf.strip_suffix("\n").unwrap().split_once(", ").unwrap();
            let usage = str::parse(usage).unwrap();
            root.add_usages(word, usage);
            buf.clear();
        }
        root
    }

    pub fn char(&self) -> char {
        char::from_u32(self.c as u32).unwrap()
    }

    fn link(&mut self, c: u8) -> &mut Node {
        self.next_count += 1;
        let index = self.next.iter().position(|n| n.c == c).unwrap_or_else(|| {
            let node = Node::new(c);
            self.next.push(node);
            self.next.len() - 1
        });
        self.next.get_mut(index).unwrap()
    }

    pub fn usages(&self) -> u32 {
        self.usage as u32 + self.next_usage
    }

    pub fn children(&self) -> SortedSliceIterator<Node> {
        SortedSliceIterator::new(&self.next)
    }

    pub fn add_usages(&mut self, word: &str, usage: u16) {
        let mut node = self;
        for ch in word.bytes() {
            node.next_usage += usage as u32;
            node = node.link(ch);
        }
        node.usage += usage;
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 5)?;
        state.serialize_field("c", &self.char())?;
        state.serialize_field("usage", &self.usage)?;
        state.serialize_field("next_count", &self.next_count)?;
        state.serialize_field("next_usage", &self.next_usage)?;
        state.serialize_field("next", &self.next)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            C,
            Usage,
            NextCount,
            NextUsage,
            Next,
        }

        struct NodeVisitor;

        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("node struct")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut c = None;
                let mut usage = None;
                let mut next_count = None;
                let mut next_usage = None;
                let mut next = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::C => {
                            if c.is_some() {
                                return Err(A::Error::duplicate_field("c"));
                            }
                            let s: String = map.next_value()?;
                            c = Some(s.bytes().next().unwrap());
                        }
                        Field::Usage => {
                            if usage.is_some() {
                                return Err(A::Error::duplicate_field("usage"));
                            }
                            usage = Some(map.next_value()?);
                        }
                        Field::NextCount => {
                            if next_count.is_some() {
                                return Err(A::Error::duplicate_field("next_count"));
                            }
                            next_count = Some(map.next_value()?);
                        }
                        Field::NextUsage => {
                            if next_usage.is_some() {
                                return Err(A::Error::duplicate_field("next_usage"));
                            }
                            next_usage = Some(map.next_value()?);
                        }
                        Field::Next => {
                            if next.is_some() {
                                return Err(A::Error::duplicate_field("next"));
                            }
                            next = Some(map.next_value()?);
                        }
                    }
                }
                let mut node = Node::new(c.ok_or(A::Error::missing_field("c"))?);
                node.usage = usage.ok_or(A::Error::missing_field("usage"))?;
                node.next_count = next_count.ok_or(A::Error::missing_field("next_count"))?;
                node.next_usage = next_usage.ok_or(A::Error::missing_field("next_usage"))?;
                node.next = next.ok_or(A::Error::missing_field("next"))?;

                Ok(node)
            }
        }
        deserializer.deserialize_map(NodeVisitor {})
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.usages().eq(&other.usages())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.usages().partial_cmp(&other.usages())
    }
}

impl<'a, T: PartialOrd> SortedSliceIterator<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        let mut order: Vec<usize> = (0..slice.len()).collect();
        order.sort_unstable_by(|a, b| -> std::cmp::Ordering {
            let t_a = slice.get(*a).unwrap();
            let t_b = slice.get(*b).unwrap();
            t_b.partial_cmp(t_a).unwrap()
        });
        Self {
            slice,
            order,
            current: 0,
        }
    }
}

impl<'a, T: PartialOrd> Iterator for SortedSliceIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.order.len() {
            None
        } else {
            let n = self
                .slice
                .get(*self.order.get(self.current).unwrap())
                .unwrap();
            self.current += 1;
            Some(n)
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::keyboard;

    use super::{Node, predict};

    fn load() -> Node {
        Node::load_dictionary_from_csv(&PathBuf::from("assets/words_with_usage.csv"))
    }

    #[test]
    fn predict_a() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let touch = (a.x + 0.1, a.y + 0.1);
        let hints = predict(&root, vec![touch]);

        assert_eq!(hints[0], "a");
    }

    #[test]
    fn predict_an() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let n = keyboard::ALPHA[24];
        let hints = predict(&root, vec![(a.x, a.y), (n.x, n.y)]);

        assert_eq!(hints[0], "an");
    }

    #[test]
    fn type_ab_but_predict_an() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let b = keyboard::ALPHA[23];
        let hints = predict(&root, vec![(a.x, a.y), (b.x, b.y)]);

        assert_eq!(hints[0], "an");
    }
}
