use std::env;
use std::io;
use std::process;

fn matches_token(ch: char, token: &str) -> bool {
    match token {
        "\\d" => ch.is_ascii_digit(),
        "\\w" => ch.is_ascii_alphabetic() || ch.is_ascii_digit(),
        token if token.starts_with("[^") && token.ends_with("]") => {
            let chars_str = &token[2..token.len() - 1];
            let chars: Vec<char> = chars_str.chars().collect();
            !chars.contains(&ch)
        }
        token if token.starts_with("[") && token.ends_with("]") => {
            let chars_str = &token[1..token.len() - 1];
            let chars: Vec<char> = chars_str.chars().collect();
            chars.contains(&ch)
        }
        _ => token.chars().next().unwrap_or('\0') == ch,
    }
}

fn matches_at_position(input_chars: &[char], tokens: &[String], start_pos: usize) -> bool {
    if start_pos + tokens.len() > input_chars.len() {
        return false;
    }
    // println!("{}: {:?} : {:?}", start_pos, tokens, input_chars);
    for (i, token) in tokens.iter().enumerate() {
        if token == "^" {
            continue;
        }

        let char_pos = start_pos + i;
        // println!("{}: {}: {}", char_pos, token, input_chars[char_pos]);
        if !matches_token(input_chars[char_pos], token) {
            return false;
        }
    }

    true
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut tokens = parse_pattern(pattern);
    let starts_with_anchor = tokens.first() == Some(&"^".to_string());

    if starts_with_anchor {
        tokens.remove(0);
    }

    // println!("{:?}", tokens);
    let input_chars: Vec<char> = input_line.chars().collect();
    // println!("{:?}", input_chars);

    if starts_with_anchor {
        matches_at_position(&input_chars, &tokens, 0)
    } else {
        for start_pos in 0..=input_chars.len() {
            if matches_at_position(&input_chars, &tokens, start_pos) {
                return true;
            }
        }
        false
    }
}

fn parse_pattern(pattern: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i < chars.len() - 1 && chars[i] == '\\' {
            let escape_seq = format!("{}{}", chars[i], chars[i + 1]);
            tokens.push(escape_seq);
            i += 2;
        } else if chars[i] == '[' {
            let start = i;
            i += 1;

            while i < chars.len() && chars[i] != ']' {
                i += 1;
            }
            if i < chars.len() {
                i += 1;
                let bracket_expr: String = chars[start..i].iter().collect();
                tokens.push(bracket_expr);
            }
        } else {
            tokens.push(chars[i].to_string());
            i += 1;
        }
    }

    tokens
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
