use std::collections::BTreeMap;

use log::debug;

use crate::{
    keyboard::{self, distances},
    node::Node,
};

const NUM_HINTS: usize = 3;
const SEARCH_RADIUS: i64 = 2000000;

pub fn multi_predict(root: &Node, points: &[(i64, i64)]) -> Vec<String> {
    let mut stack: BTreeMap<i64, Vec<&Node>> = BTreeMap::from([(0i64, vec![root])]);
    let num_points = points.len();
    for (i, (x, y)) in points.iter().enumerate() {
        let remaining_points = num_points - i - 1;
        let dist = distances(&keyboard::ALPHA, *x, *y);
        let scores = stack
            .iter()
            .rev()
            .take(NUM_HINTS)
            .flat_map(|(prev_score, nodes)| {
                nodes.last().unwrap().children().map(move |c| {
                    let mut nodes = nodes.clone();
                    nodes.push(c);
                    (prev_score, nodes)
                })
            })
            .filter_map(|(prev_score, nodes)| {
                dist.iter().find_map(move |(key, distance)| {
                    let nodes = nodes.clone();
                    if *distance > SEARCH_RADIUS {
                        return None;
                    }
                    let child = nodes.last().unwrap();
                    if child.c.eq(&key.ch) {
                        let u = child.usages_at_level(remaining_points);
                        debug!(
                            "[{i}] {child}\nlevel: {remaining_points}\nusages: {u}\ndistance: {distance}",
                        );
                        Some((
                            prev_score + u * (SEARCH_RADIUS - distance),
                            nodes,
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();
        stack.clear();
        for (mut score, mut nodes) in scores.into_iter() {
            while let Some(existing) = stack.insert(score, nodes) {
                // If there was already a value here, put it back in with a slightly higher score
                // so they stay ordered
                score += 1;
                nodes = existing;
            }
        }
        for (score, nodes) in &stack {
            let word = nodes
                .iter()
                .map(|n| n.to_string())
                .reduce(|acc, c| acc + &c)
                .unwrap_or_default();
            debug!("[{i}] {word}: {score}");
        }
    }
    stack
        .values()
        .rev()
        .take(NUM_HINTS)
        .map(|nodes| {
            nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
}

pub fn predict(root: &Node, points: &[(i64, i64)]) -> String {
    let mut guess = root.to_string();
    if let Some(((x, y), points)) = points.split_first() {
        if let Some((_score, child)) = distances(&keyboard::ALPHA, *x, *y)
            .iter()
            .filter_map(|(key, distance)| {
                if *distance > SEARCH_RADIUS {
                    return None;
                }
                root.children().find_map(|child| {
                    if child.c.eq(&key.ch) {
                        Some((child.usages() * (SEARCH_RADIUS - distance), child))
                    } else {
                        None
                    }
                })
            })
            .max_by_key(|(score, _)| *score)
        {
            guess = format!("{guess}{}", predict(child, points))
        }
    }
    guess
}

#[cfg(test)]
mod test {
    use std::sync::Once;

    use crate::{keyboard, node::Node};

    use super::{NUM_HINTS, multi_predict, predict};

    static INIT_LOG: Once = Once::new();

    fn load() -> Node {
        INIT_LOG.call_once(|| colog::basic_builder().is_test(true).init());
        Node::from_words(&[
            "a", "an", "apple", "apply", "can", "allow", "in", "un", "unto", "on", "it", "ln",
            "onto", "oh", "obelisk",
        ])
    }

    #[test]
    fn predict_a() {
        let root = load();
        let a = keyboard::ALPHA[10];
        let touch = (a.x + 100, a.y + 100);
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

    #[test]
    fn multi_predict_in_as_un_in_on() {
        let root = load();
        let i = keyboard::ALPHA[7];
        let n = keyboard::ALPHA[24];
        let hints = multi_predict(&root, &[(i.x, i.y), (n.x, n.y)]);
        assert_eq!(hints.len(), NUM_HINTS, "{:?}", &hints);
        dbg!(&hints);
        let mut iter = hints.iter();
        assert_eq!(iter.next(), Some(&"in".to_string()));
        assert_eq!(iter.next(), Some(&"on".to_string()));
        assert_eq!(iter.next(), Some(&"un".to_string()));
    }
}
