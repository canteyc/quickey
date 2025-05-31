use crate::{
    keyboard::{self, distances},
    math::cmp_f32,
    node::Node,
};

fn _predict(root: &Node, points: &[(f32, f32)]) -> Vec<String> {
    let mut results = vec![];
    let mut node = root;
    let mut buf = String::new();
    for (x, y) in points {
        let dist = distances(&keyboard::ALPHA, *x, *y);
        let probs = dist.iter().filter_map(|(k, d)| {
            let p = node.children().find_map(|child| {
                if child.c.eq(&k.ch) {
                    Some((child.usages() as f32 * (10.0f32 - d), child))
                } else {
                    None
                }
            });
            p.map(|p| (k, p.0, p.1))
        });
        if let Some((key, _, child)) = probs.max_by(|a, b| cmp_f32(a.1, b.1)) {
            buf.push(char::from_u32(key.ch as u32).unwrap());
            node = child;
        }
    }
    results.push(buf);
    results
}

pub fn predict(root: &Node, points: &[(f32, f32)]) -> String {
    let mut guess = root.to_string();
    if let Some(((x, y), points)) = points.split_first() {
        let dist = distances(&keyboard::ALPHA, *x, *y);
        if let Some((_score, child)) = dist
            .iter()
            .filter_map(|(key, distance)| {
                root.children().find_map(|child| {
                    if child.c.eq(&key.ch) {
                        Some((child.usages() as f32 * (10f32 - distance), child))
                    } else {
                        None
                    }
                })
            })
            .max_by(|a, b| cmp_f32(a.0, b.0))
        {
            guess = format!("{guess}{}", predict(child, points))
        }
    }
    guess
}

#[cfg(test)]
mod test {
    use crate::{keyboard, node::Node};

    use super::predict;

    fn load() -> Node {
        Node::from_words(&["a", "an", "apple", "apply", "can", "allow"])
    }

    #[test]
    fn predict_a() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let touch = (a.x + 0.1, a.y + 0.1);
        let hints = predict(&root, &[touch]);

        assert_eq!(hints, "a");
    }

    #[test]
    fn predict_an() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let n = keyboard::ALPHA[24];
        let hints = predict(&root, &[(a.x, a.y), (n.x, n.y)]);

        assert_eq!(hints, "an");
    }

    #[test]
    fn type_ab_but_predict_an() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let b = keyboard::ALPHA[23];
        let hints = predict(&root, &[(a.x, a.y), (b.x, b.y)]);
        assert_eq!(hints, "an");
    }

    #[test]
    fn type_aollr_but_predict_apple() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let o = keyboard::ALPHA[8];
        let l = keyboard::ALPHA[18];
        let r = keyboard::ALPHA[3];
        let hints = predict(
            &root,
            &[(a.x, a.y), (o.x, o.y), (l.x, l.y), (l.x, l.y), (r.x, r.y)],
        );
        assert_eq!(hints, "apple");
    }
}
