use serde::{Deserialize, Serialize};

use crate::math::distance_sq;

pub const ROW_X: [i64; 3] = [0, 200, 700];
pub const ROX_Y: [i64; 3] = [0, 1000, 2000];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub ch: u8,
    pub x: i64,
    pub y: i64,
}

impl Key {
    pub const fn new(ch: u8, col: usize, row: usize) -> Self {
        Key {
            ch,
            x: ROW_X[row] + (col as i64 * 1000),
            y: ROX_Y[row],
        }
    }

    pub fn xy(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

pub mod qwerty {
    use super::Key;
    //    Key { ch: b'`', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'1', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'2', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'3', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'4', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'5', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'6', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'7', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'8', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'9', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'0', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'-', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'=', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'BACKSPACE', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'\t', shifted: b'~', x: 0000, y: 0000 },
    pub const Q: Key = Key::new(b'q', 0, 0);
    pub const W: Key = Key::new(b'w', 1, 0);
    pub const E: Key = Key::new(b'e', 2, 0);
    pub const R: Key = Key::new(b'r', 3, 0);
    pub const T: Key = Key::new(b't', 4, 0);
    pub const Y: Key = Key::new(b'y', 5, 0);
    pub const U: Key = Key::new(b'u', 6, 0);
    pub const I: Key = Key::new(b'i', 7, 0);
    pub const O: Key = Key::new(b'o', 8, 0);
    pub const P: Key = Key::new(b'p', 9, 0);
    //    Key { ch: b'[', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b']', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'\', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'CAPS', shifted: b'~', x: 0000, y: 0000 },
    pub const A: Key = Key::new(b'a', 0, 1);
    pub const S: Key = Key::new(b's', 1, 1);
    pub const D: Key = Key::new(b'd', 2, 1);
    pub const F: Key = Key::new(b'f', 3, 1);
    pub const G: Key = Key::new(b'g', 4, 1);
    pub const H: Key = Key::new(b'h', 5, 1);
    pub const J: Key = Key::new(b'j', 6, 1);
    pub const K: Key = Key::new(b'k', 7, 1);
    pub const L: Key = Key::new(b'l', 8, 1);
    //    Key { ch: b';', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b''', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'\n', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'LSHIFT', shifted: b'~', x: 0000, y: 0000 },
    pub const Z: Key = Key::new(b'z', 0, 2);
    pub const X: Key = Key::new(b'x', 1, 2);
    pub const C: Key = Key::new(b'c', 2, 2);
    pub const V: Key = Key::new(b'v', 3, 2);
    pub const B: Key = Key::new(b'b', 4, 2);
    pub const N: Key = Key::new(b'n', 5, 2);
    pub const M: Key = Key::new(b'm', 6, 2);
    //    Key { ch: b',', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'.', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'/', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'RSHIFT', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'CTRL', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'FN', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'SUPER', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'ALT', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b' ', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'ALT', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'CTRL', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'UP', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'DOWN', shifted: b'~', x: 0000, y: 2000 },
    //    Key { ch: b'LEFT', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'RIGHT', shifted: b'~', x: 0000, y: 0000 },
    pub const LAYOUT: [Key; 26] = [
        Q, W, E, R, T, Y, U, I, O, P, A, S, D, F, G, H, J, K, L, Z, X, C, V, B, N, M,
    ];
}

pub fn distances(keys: &[Key], x: i64, y: i64) -> Vec<(Key, i64)> {
    keys.iter()
        .map(|k| (*k, distance_sq(k.xy(), (x, y))))
        .collect()
}

pub fn nearest(keys: &[Key], x: i64, y: i64) -> Key {
    sorted_distances(keys, x, y)[0].0
}

pub fn sorted_distances(keys: &[Key], x: i64, y: i64) -> Vec<(Key, i64)> {
    let mut dist = distances(keys, x, y);
    dist.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Less));
    dist
}

#[cfg(test)]
mod test {
    use crate::keyboard::sorted_distances;

    use super::{nearest, qwerty};

    #[test]
    fn nearest_on_key() {
        let touch = (3200, 1000); // f key
        let dist = nearest(&qwerty::LAYOUT, touch.0, touch.1);
        assert_eq!(dist, qwerty::F);
    }

    #[test]
    fn nearest_on_row() {
        let touch = (3000, 1000); // between d and f key
        let dist = nearest(&qwerty::LAYOUT, touch.0, touch.1);
        assert_eq!(dist, qwerty::F);
    }

    #[test]
    fn nearest_between_rows() {
        let touch = (3000, 800); // f key
        let dist = nearest(&qwerty::LAYOUT, touch.0, touch.1);
        assert_eq!(dist, qwerty::F);
    }

    #[test]
    fn distances() {
        let touch = (3000, 900); // f key
        let dist = sorted_distances(&qwerty::LAYOUT, touch.0, touch.1);
        assert_eq!(dist[0].0, qwerty::F);
        assert_eq!(dist[0].1, 50000, "{}", dist[0].1);

        assert_eq!(dist[1].0, qwerty::D);
        assert_eq!(dist[1].1, 650000, "{}", dist[1].1);

        assert_eq!(dist[2].0, qwerty::R);
        assert_eq!(dist[2].1, 810000, "{}", dist[2].1);

        assert_eq!(dist[3].0, qwerty::C);
        assert_eq!(dist[3].1, 1300000, "{}", dist[3].1);

        assert_eq!(dist[4].0, qwerty::G);
        assert_eq!(dist[4].1, 1450000, "{}", dist[4].1);
    }
}
