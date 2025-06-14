use serde::{Deserialize, Serialize};

use crate::math::Point;

pub const ROW_X: [i64; 3] = [0, 200, 700];
pub const ROX_Y: [i64; 3] = [0, 1000, 2000];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub ch: u8,
    pub loc: Point,
}

impl Key {
    pub const fn new(ch: u8, col: usize, row: usize) -> Self {
        Key {
            ch,
            loc: Point(ROW_X[row] + (col as i64 * 1000), ROX_Y[row]),
        }
    }
}

pub mod qwerty {
    use phf::phf_map;

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

    pub static MAP: phf::Map<u8, Key> = phf_map! {
        b'q' => Q,
        b'w' => W,
        b'e' => E,
        b'r' => R,
        b't' => T,
        b'y' => Y,
        b'u' => U,
        b'i' => I,
        b'o' => O,
        b'p' => P,
        b'a' => A,
        b's' => S,
        b'd' => D,
        b'f' => F,
        b'g' => G,
        b'h' => H,
        b'j' => J,
        b'k' => K,
        b'l' => L,
        b'z' => Z,
        b'x' => X,
        b'c' => C,
        b'v' => V,
        b'b' => B,
        b'n' => N,
        b'm' => M,
    };
}

pub fn distances(keys: &[Key], point: Point) -> Vec<(Key, i64)> {
    keys.iter().map(|k| (*k, k.loc.dist_sq(point))).collect()
}

pub fn nearest(keys: &[Key], point: Point) -> Key {
    sorted_distances(keys, point)[0].0
}

pub fn sorted_distances(keys: &[Key], point: Point) -> Vec<(Key, i64)> {
    let mut dist = distances(keys, point);
    dist.sort_unstable_by_key(|a| a.1);
    dist
}

#[cfg(test)]
mod test {
    use crate::{keyboard::sorted_distances, math::Point};

    use super::{nearest, qwerty};

    #[test]
    fn nearest_on_key() {
        let touch = Point(3200, 1000); // f key
        let dist = nearest(&qwerty::LAYOUT, touch);
        assert_eq!(dist, qwerty::F);
    }

    #[test]
    fn nearest_on_row() {
        let touch = Point(3000, 1000); // between d and f key
        let dist = nearest(&qwerty::LAYOUT, touch);
        assert_eq!(dist, qwerty::F);
    }

    #[test]
    fn nearest_between_rows() {
        let touch = Point(3000, 800); // f key
        let dist = nearest(&qwerty::LAYOUT, touch);
        assert_eq!(dist, qwerty::F);
    }

    #[test]
    fn distances() {
        let touch = Point(3000, 900); // f key
        let dist = sorted_distances(&qwerty::LAYOUT, touch);
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
