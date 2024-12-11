use crate::token::{CharacterType, Token, TokenRange, TokenType};
use colored::Colorize;

fn read_file(path: &str) -> Vec<char> {
    let code = std::fs::read_to_string(path).expect("File not found");
    code.chars().collect()
}

fn get_character(code: &Vec<char>, index: usize) -> Option<CharacterType> {
    code.get(index).map(|c| match c {
        ' ' | '\t' => CharacterType::WhiteSpace,
        '\n' => CharacterType::NewLine,
        'a'..='z' | 'A'..='Z' | '_' => CharacterType::NonDigit,
        '0'..='9' => CharacterType::Digit,
        _ => CharacterType::NonAlpha(*c),
        // todo: ignore comments
    })
}

fn find_end_of_token(code: &Vec<char>, index: usize, f: fn(char) -> bool) -> usize {
    let mut len = 1;
    while let Some(c) = code.get(index + len) {
        if f(*c) {
            len += 1;
        } else {
            break;
        }
    }
    len
}

fn error(
    path: &str,
    code: &Vec<char>,
    index: usize,
    line: usize,
    line_start: usize,
    msg: &str,
    suggest: &str,
) {
    //Error message
    println!("{}: {}", "lexer error".red().bold(), msg.bold());
    println!(
        "  {} {}:{}:{}",
        "-->".blue().bold(),
        path,
        line,
        index - line_start + 1
    );
    println!("   {}", "|".blue().bold());
    println!(
        "{:3}{} {}",
        line.to_string().blue().bold(),
        "|".blue().bold(),
        code[line_start..]
            .iter()
            .take_while(|&&c| c != '\n')
            .collect::<String>()
            .trim_end()
            .bold()
    );

    //Suggestion message
    print!("   {}", "|".blue().bold());
    for _ in 0..index - line_start + 1 {
        print!("{}", ' ');
    }
    println!("{} {}", "^".red().bold(), suggest.red().bold());
    println!("   {}", "|".blue().bold());
}

pub fn tokenize(path: &str) -> Vec<Token> {
    let code = read_file(path);
    let mut index = 0;
    let mut line_start = 0;
    let mut line = 1;
    let mut tokens = Vec::new();

    while let Some(character) = get_character(&code, index) {
        match character {
            CharacterType::WhiteSpace => {
                index += 1;
            }
            CharacterType::NewLine => {
                index += 1;
                line += 1;
                line_start = index;
            }
            CharacterType::NonDigit => {
                let len = find_end_of_token(&code, index, |c| match c {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
                    _ => false,
                });
                let name = code[index..index + len].iter().collect::<String>();
                let range = TokenRange::from(index, len, line_start);
                if let Some(keyword) = TokenType::from_keyword(&name) {
                    tokens.push(Token::new(keyword, line, range));
                } else {
                    tokens.push(Token::new(TokenType::Ident(name), line, range));
                }
                index += len;
            }
            CharacterType::Digit => {
                // println!("digit: {:?}", code[index]);
                let base = match code.get(index..index + 2) {
                    Some(&['0', 'x']) | Some(&['0', 'X']) => {
                        index += 2;
                        16
                    }
                    Some(&['0', _]) => 8,
                    _ => 10,
                };
                let len = find_end_of_token(&code, index, |c| match c {
                    ';' => false,
                    _ => true,
                });
                let number: String = code[index..index + len].iter().collect();
                let range = TokenRange::from(index, len, line_start);
                // println!("number {}", number);
                let num = match number.parse() {
                    Ok(x) => x,
                    Err(_) => 0, // Wrong number set to zero
                };
                let token = Token::new(TokenType::Number(num), line, range);
                tokens.push(token);
                index += len;

                if !check_num(&number, base) {
                    let msg = match base {
                        16 => format!("Invalid Hexadecimal Number {}", number),
                        8 => format!("Invalid Octal Number {}", number),
                        10 => format!("Invalid Decimal Number {}", number),
                        _ => {
                            unreachable!()
                        }
                    };
                    error(
                        path,
                        &code,
                        index,
                        line,
                        line_start,
                        &msg,
                        "Check if it is valid",
                    );
                }
            }
            CharacterType::NonAlpha(_) => {
                // todo:支持（多行）注释、除法

                // println!("char: {:?}", code[index]);

                if let Some(symbol) = code.get(index..index + 2) {
                    let symbol = symbol.iter().collect::<String>();
                    if let Some(symbol) = TokenType::from_double_symbol(&symbol) {
                        let range = TokenRange::from(index, 2, line_start);
                        tokens.push(Token::new(symbol, line, range));
                        index += 2;
                        continue;
                    }
                }

                if let Some(symbol) = code.get(index) {
                    if let Some(symbol) = TokenType::from_single_symbol(*symbol) {
                        let range = TokenRange::from(index, 1, line_start);
                        tokens.push(Token::new(symbol, line, range));
                        index += 1;
                    } else {
                        error(
                            path,
                            &code,
                            index,
                            line,
                            line_start,
                            &format!("Invalid Symbol {}", symbol),
                            "Check the symbol is valid",
                        );
                        index += 1;
                    }
                }
            }
        }
    }

    tokens
}

fn check_num(number: &str, base: usize) -> bool {
    number.chars().all(|c| c.to_digit(base as u32).is_some())
}
