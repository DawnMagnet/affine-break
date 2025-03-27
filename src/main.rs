use clap::Parser;
use std::process;

const fn mod_space(x: i32) -> i32 {
    x.rem_euclid(26)
}

const fn char_to_num(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32
    } else {
        c as i32 - 'A' as i32
    }
}

const fn solve_a_and_b(c1: char, x1: char, c2: char, x2: char) -> Option<(i32, i32)> {
    let diff_c = char_to_num(c1) - char_to_num(c2);
    let diff_x = char_to_num(x1) - char_to_num(x2);

    let mut a = 0;
    while a < 26 {
        if mod_space(diff_x * a) == mod_space(diff_c) {
            let b = mod_space(char_to_num(c1) - (a * char_to_num(x1)));
            return Some((a, b));
        }
        a += 1;
    }

    None
}

fn affine_decrypt(info: &str, a: i32, b: i32) -> String {
    let mut inverse_map = [0; 26];

    for i in 0..26 {
        inverse_map[mod_space(a * i + b) as usize] = i;
    }

    let mut result = String::new();

    for x in info.chars() {
        if x.is_ascii_alphabetic() {
            let num = inverse_map[char_to_num(x) as usize];
            if x.is_ascii_lowercase() {
                result.push((b'a' + num as u8) as char);
            } else {
                result.push((b'A' + num as u8) as char);
            }
        } else {
            result.push(x);
        }
    }

    result
}

#[derive(Parser)]
#[command(
    author = "DawnMagnet",
    version,
    about = "Decrypt a ciphertext using the affine cipher
you should provide two pairs of ciphertext and plaintext characters"
)]
struct Args {
    /// The ciphertext to decrypt
    ciphertext: String,
    /// The first ciphertext character
    c1: char,
    /// The first plaintext character
    x1: char,
    /// The second ciphertext character
    c2: char,
    /// The second plaintext character
    x2: char,
}

fn main() {
    let args = Args::parse();

    match solve_a_and_b(args.c1, args.x1, args.c2, args.x2) {
        Some(result) => {
            let res = affine_decrypt(&args.ciphertext, result.0, result.1);
            println!("{}", res);
        }
        None => {
            eprintln!("No solution found");
            process::exit(1);
        }
    }
}

// test cases

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt() {
        let entrypted = "prwy{w14wf3p5-fo6w-11gf-w02o-88g9pg5197wo}";
        let res = solve_a_and_b('p', 'f', 'r', 'l');
        // cause prwy -> flag, so you can pick any two characters from the ciphertext and plaintext
        assert_eq!(res, Some((9, 22)));
        let (a, b) = res.unwrap();
        let dec = affine_decrypt(entrypted, a, b);
        assert_eq!(dec, "flag{a14ab3f5-bc6a-11eb-a02c-88e9fe5197ac}");
    }
}
