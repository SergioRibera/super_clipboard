use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

use crate::settings::PasswordGenSettings;

pub const SPECIAL_CHARS: [char; 18] = [
    '!', '@', '#', '$', '%', '&', '^', '*', ']', '[', '(', ')', '{', '}', '+', '-', 'ñ', 'Ñ',
];

pub struct PasswordGenerator {
    chars: Vec<char>,
    range: Uniform<usize>,
    rng: StdRng,
}

impl PasswordGenerator {
    pub fn new(config: &PasswordGenSettings) -> Self {
        let mut chars: Vec<char> = Vec::new();

        if config.special {
            chars.append(&mut SPECIAL_CHARS.to_vec());
        }

        if config.upper {
            let mut upper = ('A'..='Z').collect();
            chars.append(&mut upper);
        }

        if config.lower {
            let mut lower = ('a'..='z').collect();
            chars.append(&mut lower);
        }

        if config.number {
            let mut number = ('0'..='9').collect();
            chars.append(&mut number);
        }

        let alphabet_length = chars.len();
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);

        Self {
            chars,
            range: Uniform::new(0, alphabet_length),
            rng: StdRng::from_seed(seed),
        }
    }

    pub fn get_char(&mut self) -> char {
        self.chars[self.rng.sample(self.range)]
    }

    pub fn generate_password(&mut self, pswd_len: usize) -> String {
        (0..pswd_len).map(|_| self.get_char()).collect()
    }
}
