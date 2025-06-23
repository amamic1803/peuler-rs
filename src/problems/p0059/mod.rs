use crate::Problem;
use std::str;

problem!(Problem0059, 59, "XOR Decryption");

impl Problem for Problem0059 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let input = include_str!("0059_cipher.txt");
        let bytes = parse_input(input);
        let mut decrypted = bytes.clone();
        let mut key = [0u8; 3];
        let mut sum: u32 = 0;

        'outer: for a in b'a'..(b'z' + 1) {
            key[0] = a;
            for b in b'a'..(b'z' + 1) {
                key[1] = b;
                for c in b'a'..(b'z' + 1) {
                    key[2] = c;
                    decrypt(&bytes, &mut decrypted, &key);
                    let decrypted_str = unsafe { str::from_utf8_unchecked(&decrypted) };
                    if decrypted_str.contains(" the ") {
                        sum = decrypted.iter().map(|&b| b as u32).sum();
                        break 'outer;
                    }
                }
            }
        }

        sum.to_string()
    }
}

fn parse_input(input: &str) -> Vec<u8> {
    input.split(',').map(|s| s.parse::<u8>().unwrap()).collect()
}

fn decrypt(bytes: &[u8], decrypted: &mut [u8], key: &[u8]) {
    for (i, d_byte) in decrypted.iter_mut().enumerate() {
        *d_byte = bytes[i] ^ key[i % key.len()];
    }
}
