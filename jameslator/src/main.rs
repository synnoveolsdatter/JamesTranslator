mod defs;
use crate::defs::*;
use std::io;
// META: Converts text to an equivalent where each consonant is represented by the current index of
// the variable `CONSANTS` repeated however many times it takes to increment through the English consonants in order to get to it.
// Same for vowels.

// const CONSONANTS: [char; 20] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];
// const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
// const JCONSANTS: [char; 3] = ['j', 'm', 's'];
// const JVOWELS: [char; 2] = ['a', 'e'];

// #[derive(PartialEq)]
// enum TranslationType {
//     JtoE,
//     EtoJ,
//     QUIT,
//     NONE,// NOTE - NONE is never used since it's just a temporary value to use in velg()
// }

fn main() {
    if cfg!(target_os = "windows") {
        let x = std::process::Command::new("cmd")
            .args(["/C", "clear"])
            .output();
        println!("{}", std::str::from_utf8(&x.unwrap().stdout).expect("oops"));
    } else {
        let x = std::process::Command::new("sh")
            .arg("-c")
            .arg("clear")
            .output();
        println!("{}", std::str::from_utf8(&x.unwrap().stdout).expect("oops"));
    }

    println!(" James-English or English-James translator");
    println!("-------------------------------------------");
    let mut kind = velg();
    while kind != TranslationType::QUIT {
        match kind {
            TranslationType::JtoE => {// note to self James to English - jtoe()
                println!("\n--------------\nJames -> English selected.");
                println!("Please enter the Jamish text you'd like translated.\n");
                let inp = {// read input
                    let mut ln = String::new();
                    io::stdin().read_line(&mut ln).unwrap();
                    ln
                }.trim().to_string();

                println!("\n");
                println!("{}\n", jtoe(inp));
            },
            TranslationType::EtoJ => {
                println!("\n--------------\nEnglish -> James selected.");
                println!("Please enter the English text you'd like translated.\n");
                let inp = {// read input
                    let mut ln = String::new();
                    io::stdin().read_line(&mut ln).unwrap();
                    ln
                }.trim().to_string();

                println!("\n");
                println!("{}\n", etoj(inp));
            },
            _ => {},
        };
        kind = velg();
    }
    println!("\nThanks for using the James-English translator, please come again!\n");
}

// James to English
#[allow(dead_code)]
fn jtoe(text: String) -> String {
    let text_bytes = text.as_bytes();

    let mut ret = "".to_string();
    let mut txt: Vec<(char, usize)> = vec![];// chars and their lengths

    // serialise jamish text into `txt` variable
    let mut i = 0;
    while i < text.len() - 1 {
        let mut j = i;
        while j < text.len() && text_bytes[j] == text_bytes[i] {
            j += 1;
        }
        txt.push((text_bytes[i] as char, j - i));
        i += j - i;// increase by the length we just pushed to the vector
    }
    
    // now that it's serialised, convert it into normal text (stored in `ret`)
    for (ch, len) in txt {
        if !ch.is_alphabetic() {
            ret.push(ch);
            continue;
        }
        // it IS alphabetic, and give nthat it's jamish, it should hopefully work w the methods but
        // we'll see ig
        ret.push( Jimser::james_to_char(ch, len) );
    }

    ret
}

// English to James
#[allow(dead_code)]
fn etoj(text: String) -> String {
    let mut jimmer = Jimser::new();
    let mut ret = "".to_string();
    let txt = text.trim().to_ascii_lowercase();

    for ch in txt.chars() {
        let mut append = format!("{}", ch);
        if ch.is_alphabetic() {
            append = jimmer.char_to_james(&ch);
        }
        ret.push_str(&append);
    }

    ret
}

// Returns type of translation desired by the user.
// It's guaranteed that NONE will never be returned
fn velg() -> TranslationType {
    let mut kind = TranslationType::NONE;
    while kind == TranslationType::NONE {
        println!("Please type: ");
        println!("-   J - James to English translation");
        println!("-   E - English to James translation");
        println!("-   q - Quit");
        let inp = {// read input
            let mut ln = String::new();
            io::stdin().read_line(&mut ln).unwrap();
            ln
        };
        match inp.trim_start()// making it easier to deal w
            .to_ascii_lowercase()
            .chars()
            .next() {
            None => {
                continue;
            },
            Some(c) => {
                kind = match c {// literally js doing what the print statement up there said, but i heard
                                // leaving comments is good practice lol
                    'j' => TranslationType::JtoE,
                    'e' => TranslationType::EtoJ,
                    'q' => TranslationType::QUIT,
                    _ => TranslationType::NONE,
                };
                if kind == TranslationType::NONE {
                    println!("\nTranslation type not recognised, please try again or enter q to quit.\n");
                }
            }
        };
    }

    kind
}
