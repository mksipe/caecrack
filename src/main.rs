#[macro_use]
extern crate clap;
use clap::App;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let yaml = load_yaml!("../src/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    caesar_crack(matches.value_of("cipher").unwrap(), matches.value_of("wordlist").unwrap())
}

fn caesar(cipher: &str, shift:u8) -> String {
    cipher.chars()
    .map(|c| {
        if c.is_ascii_alphabetic(){
            let first = if c.is_ascii_lowercase(){b'a'} else {b'A'};
            (first + (c as u8 + shift - first) % 26) as char
        } else {
            c
        }
    }).collect()
}

fn caesar_crack (word:&str, list:&str) {
    let fcheck = std::path::Path::new(&list).exists();

    if fcheck == false {
        println!("{}", "File does not exist.\n");
        panic!();
    }

    let file = std::fs::File::open(&list).unwrap();

    let reader = BufReader::new(file);
    println!("{}", "Cracking... ");
    for i in reader.lines().enumerate() {
    let word = word.split_whitespace().next().unwrap_or("");
    let poss:[&str; 26] = [&caesar(word,1),&caesar(word,2),&caesar(word,3),&caesar(word,4),&caesar(word,5),&caesar(word,6),&caesar(word,7),&caesar(word,8),&caesar(word,9),&caesar(word,10),&caesar(word,11),&caesar(word,12),&caesar(word,13),&caesar(word,14),&caesar(word,15),&caesar(word,16),&caesar(word,17),&caesar(word,18),&caesar(word,19),&caesar(word,20),&caesar(word,21),&caesar(word,22),&caesar(word,23),&caesar(word,24),&caesar(word,25),&caesar(word,26)];
    let i = i.1.unwrap();
    let out:[&bool; 26] = [&poss[0].eq(&i),&poss[1].eq(&i),&poss[2].eq(&i),&poss[3].eq(&i),&poss[4].eq(&i),&poss[5].eq(&i),&poss[6].eq(&i),&poss[7].eq(&i),&poss[8].eq(&i),&poss[9].eq(&i),&poss[10].eq(&i),&poss[11].eq(&i),&poss[12].eq(&i),&poss[13].eq(&i),&poss[14].eq(&i),&poss[15].eq(&i),&poss[16].eq(&i),&poss[17].eq(&i),&poss[18].eq(&i),&poss[19].eq(&i),&poss[20].eq(&i),&poss[21].eq(&i),&poss[22].eq(&i),&poss[23].eq(&i),&poss[24].eq(&i),&poss[25].eq(&i)];
    let out = out.iter().enumerate();
    for i in out {
        match i.1 {
            true => println!("Found: {} Rotations", i.0 + 1),
            _ => print!("{}", "" ),
        }
    }

}
}
