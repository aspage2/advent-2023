
use std::env;
use std::fs;

static DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn take_number(s: &Vec<u8>, idx: usize) -> Option<u32> {
    if idx >= s.len() {
        return None
    }
    let rel = (s[idx] as i32) - ('0' as i32);
    if rel >= 0 && rel <= 9 {
        return Some(rel as u32);
    }
    for (st, d) in DIGITS {
        if s[idx..].starts_with(st.as_bytes()) {
            return Some(d);
        }
    }
    return None
}

fn get_number(s: &str) -> u32 {
    let s_bytes: Vec<u8> = s.bytes().collect();

    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    let mut idx: usize = 0;

    while idx < s_bytes.len() {

        let res = take_number(&s_bytes, idx);
        match res {
            None => {},
            Some(digit) => {
                match first {
                    None => {
                        first = Some(digit);
                        last = Some(digit);
                    },
                    Some(_) => {
                        last = Some(digit)
                    }
                };
            }
        }
        idx += 1;
    }
    return 10 * first.unwrap_or_default() + last.unwrap_or_default()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please enter filename");
        return
    }
    let contents = args.last()
        .and_then(|fname| fs::read_to_string(fname).ok())
        .expect("couldn't read file");

    let mut total = 0;
    for line in contents.lines() {
        let num = get_number(line);
        println!("{} ({})", &line, &num);
        total += num;
    }

    println!("{}", total);
}
