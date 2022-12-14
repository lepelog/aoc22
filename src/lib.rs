use std::mem::swap;

pub fn parse_num_from_bytes(bytes: &[u8]) -> isize {
    let mut b_iter = bytes.iter().peekable();
    let is_negative = match b_iter.peek().copied() {
        Some(b'-') => {
            let _ = b_iter.next();
            true
        }
        _ => false,
    };
    let mut num = 0;
    for num_byte in b_iter {
        match num_byte {
            b'0'..=b'9' => {
                num = 10 * num + <u8 as Into<isize>>::into(*num_byte - b'0');
            }
            _ => break,
        }
    }
    if is_negative {
        -num
    } else {
        num
    }
}

pub struct Highest<const N: usize> {
    vals: [usize; N],
}

impl<const N: usize> Highest<N> {
    pub fn new() -> Self {
        Self { vals: [0; N] }
    }

    pub fn insert(&mut self, mut new_val: usize) {
        for old_val in self.vals.iter_mut() {
            if new_val > *old_val {
                swap(&mut new_val, old_val);
            }
        }
    }

    pub fn get(&self) -> &[usize; N] {
        &self.vals
    }
}
