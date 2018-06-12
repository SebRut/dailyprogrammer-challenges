use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn get_exceptions_in_enable1() -> Vec<String> {
    let mut result = vec![];
    let f = File::open("enable1.txt").unwrap();
    let mut file = BufReader::new(&f);

    for l in file.lines() {
        let line = l.unwrap();
        if !check(line.clone()) {
            result.push(line);
        }
    }

    result
}

    fn check(word: String) -> bool {
        for (i,c) in word.chars().enumerate(){
            let next = word.chars().nth(i+1);
            if i == 0 {
                return match c {
                    'e' => false,
                    _ => continue,
                }
            }
            let pre = word.chars().nth(i-1);
            if c == 'e' {          
                match next {
                    Some('i') => { },
                    Some(_) | None => {continue;}
                };
                match pre {
                    Some('c') => {continue;},
                    Some(_) | None => {return false;}
                };
            }
            if c == 'i' {        
                match next {
                    Some('e') => { },
                    Some(_) | None => {continue;}
                };
                match pre {
                    Some('c') => {return false;},
                    Some(_) | None => {continue;}
                };
            }
        }

        true
    }

fn main() {
    let exception_count = get_exceptions_in_enable1().len();

    println!("Exceptions in enable1: {}", exception_count);
}

#[test]
fn test_examples() {
    assert!(check("a".to_string()));
    assert!(check("zombie".to_string()));
    assert!(check("transceiver".to_string()));
    assert!(!check("veil".to_string()));
    assert!(!check("icier".to_string()));
    assert!(!check("either".to_string()));
}