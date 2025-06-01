use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub ch: u8,
    pub shifted: u8,
    pub x: i64,
    pub y: i64,
}

pub const ALPHA: [Key; 26] = [
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
    Key {
        ch: b'q',
        shifted: b'Q',
        x: 0000,
        y: 0000,
    },
    Key {
        ch: b'w',
        shifted: b'W',
        x: 1000,
        y: 0000,
    },
    Key {
        ch: b'e',
        shifted: b'E',
        x: 2000,
        y: 0000,
    },
    Key {
        ch: b'r',
        shifted: b'R',
        x: 3000,
        y: 0000,
    },
    Key {
        ch: b't',
        shifted: b'T',
        x: 4000,
        y: 0000,
    },
    Key {
        ch: b'y',
        shifted: b'Y',
        x: 5000,
        y: 0000,
    },
    Key {
        ch: b'u',
        shifted: b'U',
        x: 6000,
        y: 0000,
    },
    Key {
        ch: b'i',
        shifted: b'I',
        x: 7000,
        y: 0000,
    },
    Key {
        ch: b'o',
        shifted: b'O',
        x: 8000,
        y: 0000,
    },
    Key {
        ch: b'p',
        shifted: b'P',
        x: 9000,
        y: 0000,
    },
    //    Key { ch: b'[', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b']', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'\', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'CAPS', shifted: b'~', x: 0000, y: 0000 },
    Key {
        ch: b'a',
        shifted: b'A',
        x: 200,
        y: 1000,
    },
    Key {
        ch: b's',
        shifted: b'S',
        x: 1200,
        y: 1000,
    },
    Key {
        ch: b'd',
        shifted: b'D',
        x: 2200,
        y: 1000,
    },
    Key {
        ch: b'f',
        shifted: b'F',
        x: 3200,
        y: 1000,
    },
    Key {
        ch: b'g',
        shifted: b'G',
        x: 4200,
        y: 1000,
    },
    Key {
        ch: b'h',
        shifted: b'H',
        x: 5200,
        y: 1000,
    },
    Key {
        ch: b'j',
        shifted: b'J',
        x: 6200,
        y: 1000,
    },
    Key {
        ch: b'k',
        shifted: b'K',
        x: 7200,
        y: 1000,
    },
    Key {
        ch: b'l',
        shifted: b'L',
        x: 8200,
        y: 1000,
    },
    //    Key { ch: b';', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b''', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'\n', shifted: b'~', x: 0000, y: 0000 },
    //    Key { ch: b'LSHIFT', shifted: b'~', x: 0000, y: 0000 },
    Key {
        ch: b'z',
        shifted: b'Z',
        x: 700,
        y: 2000,
    },
    Key {
        ch: b'x',
        shifted: b'X',
        x: 1700,
        y: 2000,
    },
    Key {
        ch: b'c',
        shifted: b'C',
        x: 2700,
        y: 2000,
    },
    Key {
        ch: b'v',
        shifted: b'V',
        x: 3700,
        y: 2000,
    },
    Key {
        ch: b'b',
        shifted: b'B',
        x: 4700,
        y: 2000,
    },
    Key {
        ch: b'n',
        shifted: b'N',
        x: 5700,
        y: 2000,
    },
    Key {
        ch: b'm',
        shifted: b'M',
        x: 6700,
        y: 2000,
    },
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
];

pub fn distances(keys: &[Key], x: i64, y: i64) -> Vec<(Key, i64)> {
    keys.iter().map(|k| (*k, distance_sq(k, x, y))).collect()
}

pub fn distance_sq(key: &Key, x: i64, y: i64) -> i64 {
    (x - key.x) * (x - key.x) + (y - key.y) * (y - key.y)
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

    use super::{ALPHA, nearest};

    #[test]
    fn nearest_on_key() {
        let touch = (3200, 1000); // f key
        let dist = nearest(&ALPHA, touch.0, touch.1);
        assert_eq!(dist, ALPHA[13]);
    }

    #[test]
    fn nearest_on_row() {
        let touch = (3000, 1000); // between d and f key
        let dist = nearest(&ALPHA, touch.0, touch.1);
        assert_eq!(dist, ALPHA[13]);
    }

    #[test]
    fn nearest_between_rows() {
        let touch = (3000, 800); // f key
        let dist = nearest(&ALPHA, touch.0, touch.1);
        assert_eq!(dist, ALPHA[13]);
    }

    #[test]
    fn distances() {
        let touch = (3000, 900); // f key
        let dist = sorted_distances(&ALPHA, touch.0, touch.1);
        assert_eq!(dist[0].0, ALPHA[13]);
        assert_eq!(dist[0].1, 50000, "{}", dist[0].1);

        assert_eq!(dist[1].0, ALPHA[12]);
        assert_eq!(dist[1].1, 650000, "{}", dist[1].1);

        assert_eq!(dist[2].0, ALPHA[3]);
        assert_eq!(dist[2].1, 810000, "{}", dist[2].1);

        assert_eq!(dist[3].0, ALPHA[21]);
        assert_eq!(dist[3].1, 1300000, "{}", dist[3].1);

        assert_eq!(dist[4].0, ALPHA[14]);
        assert_eq!(dist[4].1, 1450000, "{}", dist[4].1);
    }
}
