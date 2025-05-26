#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Key {
    pub ch: u8,
    pub shifted: u8,
    pub x: f32,
    pub y: f32,
}

pub const ALPHA: [Key; 26] = [
    //    Key { ch: b'`', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'1', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'2', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'3', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'4', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'5', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'6', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'7', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'8', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'9', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'0', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'-', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'=', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'BACKSPACE', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'\t', shifted: b'~', x: 0.0, y: 0.0 },
    Key {
        ch: b'q',
        shifted: b'Q',
        x: 0.0,
        y: 0.0,
    },
    Key {
        ch: b'w',
        shifted: b'W',
        x: 1.0,
        y: 0.0,
    },
    Key {
        ch: b'e',
        shifted: b'E',
        x: 2.0,
        y: 0.0,
    },
    Key {
        ch: b'r',
        shifted: b'R',
        x: 3.0,
        y: 0.0,
    },
    Key {
        ch: b't',
        shifted: b'T',
        x: 4.0,
        y: 0.0,
    },
    Key {
        ch: b'y',
        shifted: b'Y',
        x: 5.0,
        y: 0.0,
    },
    Key {
        ch: b'u',
        shifted: b'U',
        x: 6.0,
        y: 0.0,
    },
    Key {
        ch: b'i',
        shifted: b'I',
        x: 7.0,
        y: 0.0,
    },
    Key {
        ch: b'o',
        shifted: b'O',
        x: 8.0,
        y: 0.0,
    },
    Key {
        ch: b'p',
        shifted: b'P',
        x: 9.0,
        y: 0.0,
    },
    //    Key { ch: b'[', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b']', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'\', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'CAPS', shifted: b'~', x: 0.0, y: 0.0 },
    Key {
        ch: b'a',
        shifted: b'A',
        x: 0.2,
        y: 1.0,
    },
    Key {
        ch: b's',
        shifted: b'S',
        x: 1.2,
        y: 1.0,
    },
    Key {
        ch: b'd',
        shifted: b'D',
        x: 2.2,
        y: 1.0,
    },
    Key {
        ch: b'f',
        shifted: b'F',
        x: 3.2,
        y: 1.0,
    },
    Key {
        ch: b'g',
        shifted: b'G',
        x: 4.2,
        y: 1.0,
    },
    Key {
        ch: b'h',
        shifted: b'H',
        x: 5.2,
        y: 1.0,
    },
    Key {
        ch: b'j',
        shifted: b'J',
        x: 6.2,
        y: 1.0,
    },
    Key {
        ch: b'k',
        shifted: b'K',
        x: 7.2,
        y: 1.0,
    },
    Key {
        ch: b'l',
        shifted: b'L',
        x: 8.2,
        y: 1.0,
    },
    //    Key { ch: b';', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b''', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'\n', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'LSHIFT', shifted: b'~', x: 0.0, y: 0.0 },
    Key {
        ch: b'z',
        shifted: b'Z',
        x: 0.7,
        y: 2.0,
    },
    Key {
        ch: b'x',
        shifted: b'X',
        x: 1.7,
        y: 2.0,
    },
    Key {
        ch: b'c',
        shifted: b'C',
        x: 2.7,
        y: 2.0,
    },
    Key {
        ch: b'v',
        shifted: b'V',
        x: 3.7,
        y: 2.0,
    },
    Key {
        ch: b'b',
        shifted: b'B',
        x: 4.7,
        y: 2.0,
    },
    Key {
        ch: b'n',
        shifted: b'N',
        x: 5.7,
        y: 2.0,
    },
    Key {
        ch: b'm',
        shifted: b'M',
        x: 6.7,
        y: 2.0,
    },
    //    Key { ch: b',', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'.', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'/', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'RSHIFT', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'CTRL', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'FN', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'SUPER', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'ALT', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b' ', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'ALT', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'CTRL', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'UP', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'DOWN', shifted: b'~', x: 0.0, y: 2.0 },
    //    Key { ch: b'LEFT', shifted: b'~', x: 0.0, y: 0.0 },
    //    Key { ch: b'RIGHT', shifted: b'~', x: 0.0, y: 0.0 },
];

pub fn distances(keys: &[Key], x: f32, y: f32) -> Vec<(Key, f32)> {
    keys.iter().map(|k| (*k, distance_sq(k, x, y))).collect()
}

pub fn distance_sq(key: &Key, x: f32, y: f32) -> f32 {
    (x - key.x) * (x - key.x) + (y - key.y) * (y - key.y)
}

pub fn nearest(keys: &[Key], x: f32, y: f32) -> Key {
    sorted_distances(keys, x, y)[0].0
}

pub fn sorted_distances(keys: &[Key], x: f32, y: f32) -> Vec<(Key, f32)> {
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
        let touch = (3.2f32, 1.0f32); // f key
        let dist = nearest(&ALPHA, touch.0, touch.1);
        assert_eq!(dist, ALPHA[13]);
    }

    #[test]
    fn nearest_on_row() {
        let touch = (3.0f32, 1.0f32); // between d and f key
        let dist = nearest(&ALPHA, touch.0, touch.1);
        assert_eq!(dist, ALPHA[13]);
    }

    #[test]
    fn nearest_between_rows() {
        let touch = (3.0f32, 0.8f32); // f key
        let dist = nearest(&ALPHA, touch.0, touch.1);
        assert_eq!(dist, ALPHA[13]);
    }

    #[test]
    fn distances() {
        let touch = (3.0f32, 0.9f32); // f key
        let dist = sorted_distances(&ALPHA, touch.0, touch.1);
        assert_eq!(dist[0].0, ALPHA[13]);
        assert!((dist[0].1 - 0.05f32).abs() < 1e-3, "{}", dist[0].1);

        assert_eq!(dist[1].0, ALPHA[12]);
        assert!((dist[1].1 - 0.65f32).abs() < 1e-3, "{}", dist[1].1);

        assert_eq!(dist[2].0, ALPHA[3]);
        assert!((dist[2].1 - 0.81f32).abs() < 1e-3, "{}", dist[2].1);

        assert_eq!(dist[3].0, ALPHA[21]);
        assert!((dist[3].1 - 1.3f32).abs() < 1e-3, "{}", dist[3].1);

        assert_eq!(dist[4].0, ALPHA[14]);
        assert!((dist[4].1 - 1.449f32).abs() < 1e-3, "{}", dist[4].1);
    }
}
