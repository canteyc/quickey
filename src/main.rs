use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf
};
use serde::{ser::SerializeStruct, Serialize, Serializer};


fn main() {
    let dict_file = PathBuf::from("assets/words_alpha.txt");
    let mut dict_reader = BufReader::new(File::open(&dict_file).unwrap());
    let mut count = 0;
    let mut buf = String::with_capacity(32);
    let mut root = Node::new(0u8);
    while dict_reader.read_line(&mut buf).is_ok_and(|s| s > 0) {
        let mut node = &mut root;
        for char in buf.as_bytes() {
            node = node.link(*char);
        }
        buf.clear();
        count += 1;
    }
    let mut node = &root;
    while !node.next.is_empty() {
        node = node.next.iter().max_by(|x, y| x.next_count.cmp(&y.next_count)).unwrap();
        println!("{}", &char::from_u32(node.c as u32).unwrap());
    }
        
//    println!("read {count} words");
//    println!("{}", serde_json::to_string_pretty(&root).unwrap());
}

struct Node {
    c: u8,
    usage: u16,
    next_count: u32,
    next_usage: u32,
    next: Vec<Node>,
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 5)?;
        state.serialize_field("c", &char::from_u32(self.c as u32).unwrap())?;
        state.serialize_field("usage", &self.usage)?;
        state.serialize_field("next_count", &self.next_count)?;
        state.serialize_field("next_usage", &self.next_usage)?;
        state.serialize_field("next", &self.next)?;
        state.end()
    }
}

impl Node {
    pub fn new(c: u8) -> Self {
        Self { c, usage: 1, next_count: 0, next_usage: 0, next: Default::default() }
    }

    pub fn link(&mut self, c: u8) -> &mut Node {
        self.next_count += 1;
        self.next_usage += 1;
        let index = self.next.iter().position(|n| n.c == c).unwrap_or_else(|| {
//            println!("{} -> {c}", self.c);
            let node = Node::new(c);
            self.next.push(node);
            self.next.len() - 1
        });
        self.next.get_mut(index).unwrap()
    }

    pub fn usages(&self) -> u32 {
        self.usage as u32 + self.next_usage
    }
}
