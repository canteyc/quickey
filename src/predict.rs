use std::collections::BTreeMap;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    keyboard::{self, distances},
    math::Point,
    node::{self, Node},
};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Score(i64);

pub fn search_predict(root: &Node, points: &[Point]) -> Vec<String> {
    let mut results = vec![];
    let num_points = points.len();
    let mut stack: BTreeMap<Score, Vec<&Node>> = BTreeMap::from_iter(root.children().map(|child| {
        let nodes = vec![child];
        (average_score(&nodes, points), nodes)
    }));

    for _ in 0..num_points {
        let (_, nodes) = stack.pop_last().unwrap();
        for child in nodes.last().unwrap().children() {
            let mut next_nodes = nodes.clone();
            next_nodes.push(child);
            let next_score = average_score(&next_nodes, points);
            stack.insert(next_score, next_nodes);
        }
    }
    results.extend(stack.values().rev().take(NUM_HINTS).map(|value| node::to_string(value)));

    results
}

pub fn average_score(nodes: &[&Node], points: &[Point]) -> Score {
    Score(
        nodes
            .iter()
            .zip(points)
            .map(|(node, point)| {
                let dist = if let Some(key) = keyboard::qwerty::MAP.get(&node.c) {
                    point.dist_sq(key.loc)
                } else {
                    panic!("{node}")
                };
                dbg!((node.char(), SEARCH_RADIUS - dist));
                SEARCH_RADIUS - dist
            })
            .sum::<i64>()
            / nodes.len() as i64,
    )
}

const NUM_HINTS: usize = 6;
const SEARCH_RADIUS: i64 = 2000000;

pub fn multi_predict(root: &Node, points: &[Point]) -> Vec<String> {
    let mut stack: BTreeMap<i64, Vec<&Node>> = BTreeMap::from([(0i64, vec![root])]);
    let num_points = points.len();
    for (i, point) in points.iter().enumerate() {
        let remaining_points = num_points - i - 1;
        let dist = distances(&keyboard::qwerty::LAYOUT, *point);
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

pub fn predict(root: &Node, points: &[Point]) -> String {
    let mut guess = root.to_string();
    if let Some((point, rem_points)) = points.split_first() {
        if let Some((_score, child)) = distances(&keyboard::qwerty::LAYOUT, *point)
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
            guess = format!("{guess}{}", predict(child, rem_points))
        }
    }
    guess
}

#[cfg(test)]
mod test {
    use std::sync::Once;

    use crate::{
        keyboard,
        math::Point,
        node::Node,
        predict::{SEARCH_RADIUS, Score, average_score},
    };

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
        let a = keyboard::qwerty::A;
        let touch = a.loc + Point(100, 100);
        let hints = predict(&root, &[touch]);

        assert_eq!(hints, "a");
    }

    #[test]
    fn predict_an() {
        let root = load();
        let a = keyboard::qwerty::A;
        let n = keyboard::qwerty::N;
        let hints = predict(&root, &[a.loc, n.loc]);

        assert_eq!(hints, "an");
    }

    #[test]
    fn type_ab_but_predict_an() {
        let root = load();
        let a = keyboard::qwerty::A;
        let b = keyboard::qwerty::B;
        let hints = predict(&root, &[a.loc, b.loc]);
        assert_eq!(hints, "an");
    }

    #[test]
    fn type_aollr_but_predict_apple() {
        let root = load();
        let a = keyboard::qwerty::A;
        let o = keyboard::qwerty::O;
        let l = keyboard::qwerty::L;
        let r = keyboard::qwerty::R;
        let hints = predict(&root, &[a.loc, o.loc, l.loc, l.loc, r.loc]);
        assert_eq!(hints, "apple");
    }

    #[test]
    fn multi_predict_in_as_un_in_on() {
        let root = load();
        let i = keyboard::qwerty::I;
        let n = keyboard::qwerty::N;
        let hints = multi_predict(&root, &[i.loc, n.loc]);
        assert_eq!(hints.len(), NUM_HINTS, "{:?}", &hints);
        dbg!(&hints);
        let mut iter = hints.iter();
        assert_eq!(iter.next(), Some(&"in".to_string()));
        assert_eq!(iter.next(), Some(&"on".to_string()));
        assert_eq!(iter.next(), Some(&"un".to_string()));
    }

    #[test]
    fn average_score_for_aollr_to_apple() {
        let nodes = vec![
            Node::new(b'a'),
            Node::new(b'p'),
            Node::new(b'p'),
            Node::new(b'l'),
            Node::new(b'e'),
        ];
        let nodes = nodes.iter().collect::<Vec<&Node>>();

        let points = vec![
            keyboard::qwerty::A.loc,
            keyboard::qwerty::O.loc,
            keyboard::qwerty::L.loc,
            keyboard::qwerty::L.loc,
            keyboard::qwerty::R.loc,
        ];

        let expected = Score(1272000);
        let score = average_score(&nodes, &points);

        assert_eq!(score, expected);
    }
}
