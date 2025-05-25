#[derive(Debug, Clone, Copy)]
pub struct Key {
    ch: u8,
    shifted: u8,
    x: f32,
    y: f32,
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
    distances(keys, x, y)
        .iter()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
        .unwrap()
        .0
}
