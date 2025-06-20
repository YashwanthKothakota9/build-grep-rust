use std::env;
use std::io;
use std::process;

#[derive(Debug, Clone)]
enum Token {
    Literal(char),
    Digit,
    Word,
    CharClass(Vec<char>),
    NegCharClass(Vec<char>),
    Plus(Box<Token>),
}

fn matches_token(ch: char, token: &Token) -> bool {
    match token {
        Token::Literal(expected) => ch == *expected,
        Token::Digit => ch.is_ascii_digit(),
        Token::Word => ch.is_ascii_alphabetic() || ch.is_ascii_digit(),
        Token::CharClass(chars) => chars.contains(&ch),
        Token::NegCharClass(chars) => !chars.contains(&ch),
        // Plus tokens can't be matched with single character matches
        Token::Plus(_) => false,
    }
}

fn matches_at_position(input_chars: &[char], tokens: &[Token], start_pos: usize) -> Option<usize> {
    matches_at_position_recursive(input_chars, tokens, start_pos, 0)
}

fn matches_at_position_recursive(
    input_chars: &[char],
    tokens: &[Token],
    pos: usize,
    token_idx: usize,
) -> Option<usize> {
    if token_idx >= tokens.len() {
        return Some(pos);
    }

    match &tokens[token_idx] {
        Token::Plus(inner_token) => {
            if pos >= input_chars.len() || !matches_token(input_chars[pos], inner_token) {
                return None;
            }

            let mut max_matches = 1;
            while pos + max_matches < input_chars.len()
                && matches_token(input_chars[pos + max_matches], inner_token)
            {
                max_matches += 1;
            }

            // Backtracking
            for num_matches in (1..=max_matches).rev() {
                if let Some(end_pos) = matches_at_position_recursive(
                    input_chars,
                    tokens,
                    pos + num_matches,
                    token_idx + 1,
                ) {
                    return Some(end_pos);
                }
            }
            None
        }
        _ => {
            if pos >= input_chars.len() || !matches_token(input_chars[pos], &tokens[token_idx]) {
                return None;
            }
            matches_at_position_recursive(input_chars, tokens, pos + 1, token_idx + 1)
        }
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut tokens = parse_pattern(pattern);
    let starts_with_anchor = pattern.starts_with('^');
    let ends_with_anchor = pattern.ends_with('$');

    if starts_with_anchor {
        if let Some(Token::Literal('^')) = tokens.first() {
            tokens.remove(0);
        }
    }

    if ends_with_anchor {
        if let Some(Token::Literal('$')) = tokens.last() {
            tokens.pop();
        }
    }

    let input_chars: Vec<char> = input_line.chars().collect();

    if starts_with_anchor && ends_with_anchor {
        if let Some(end_pos) = matches_at_position(&input_chars, &tokens, 0) {
            return end_pos == input_chars.len();
        }
        false
    } else if starts_with_anchor {
        return matches_at_position(&input_chars, &tokens, 0).is_some();
    } else if ends_with_anchor {
        for start_pos in 0..=input_chars.len() {
            if let Some(end_pos) = matches_at_position(&input_chars, &tokens, start_pos) {
                if end_pos == input_chars.len() {
                    return true;
                }
            }
        }
        return false;
    } else {
        for start_pos in 0..=input_chars.len() {
            if matches_at_position(&input_chars, &tokens, start_pos).is_some() {
                return true;
            }
        }
        return false;
    }
}

fn parse_pattern(pattern: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i < chars.len() - 1 && chars[i] == '\\' {
            let token = match chars[i + 1] {
                'd' => Token::Digit,
                'w' => Token::Word,
                c => Token::Literal(c),
            };
            tokens.push(token);
            i += 2;
        } else if chars[i] == '[' {
            i += 1;

            let negated = i < chars.len() && chars[i] == '^';
            if negated {
                i += 1;
            }

            let mut char_class = Vec::new();
            while i < chars.len() && chars[i] != ']' {
                char_class.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                i += 1;
                let token = if negated {
                    Token::NegCharClass(char_class)
                } else {
                    Token::CharClass(char_class)
                };
                tokens.push(token);
            }
        } else if chars[i] == '+' {
            if let Some(last_token) = tokens.pop() {
                tokens.push(Token::Plus(Box::new(last_token)));
            }
            i += 1;
        } else {
            tokens.push(Token::Literal(chars[i]));
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

    if input_line.ends_with('\n') {
        input_line.pop();
    }

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
