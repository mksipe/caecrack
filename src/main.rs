#[macro_use]
extern crate clap;
use clap::App;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let yaml = load_yaml!("../src/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    caesar_crack(matches.value_of("cipher").unwrap(), matches.value_of("wordlist").unwrap());
    println!("{}", "Could not find a match.");

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
        println!("{}", "File does not exist.");
        std::process::exit(1);
    }

    let file = std::fs::File::open(&list).unwrap();
    let reader = BufReader::new(file);
    println!("{}", "Cracking... ");
    for i in reader.lines().enumerate() {
    let word = word.split_whitespace().next().unwrap_or("");
    let word = &word.to_lowercase();
    let poss:[&str; 26] = [&caesar(&word,1),&caesar(&word,2),&caesar(&word,3),&caesar(&word,4),&caesar(&word,5),&caesar(&word,6),&caesar(&word,7),&caesar(&word,8),&caesar(&word,9),&caesar(&word,10),&caesar(&word,11),&caesar(&word,12),&caesar(&word,13),&caesar(&word,14),&caesar(&word,15),&caesar(&word,16),&caesar(&word,17),&caesar(&word,18),&caesar(&word,19),&caesar(&word,20),&caesar(&word,21),&caesar(&word,22),&caesar(&word,23),&caesar(&word,24),&caesar(&word,25),&caesar(&word,26)];
    let i = i.1.unwrap();
    let out:[&bool; 26] = [&poss[0].eq(&i),&poss[1].eq(&i),&poss[2].eq(&i),&poss[3].eq(&i),&poss[4].eq(&i),&poss[5].eq(&i),&poss[6].eq(&i),&poss[7].eq(&i),&poss[8].eq(&i),&poss[9].eq(&i),&poss[10].eq(&i),&poss[11].eq(&i),&poss[12].eq(&i),&poss[13].eq(&i),&poss[14].eq(&i),&poss[15].eq(&i),&poss[16].eq(&i),&poss[17].eq(&i),&poss[18].eq(&i),&poss[19].eq(&i),&poss[20].eq(&i),&poss[21].eq(&i),&poss[22].eq(&i),&poss[23].eq(&i),&poss[24].eq(&i),&poss[25].eq(&i)];
    let out = out.iter().enumerate();
    for i in out {
        match &i.1 {
            true  => println!("Found: {} Rotations", &i.0 + 1),
            false => print!("{}", "" ),
        }
        match &i.1 {
            true  => (std::process::exit(0)),
            false => print!("{}", "" ),
        }
    }  

}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot_0() {
        assert_eq!(caesar("test",0), "test");
    }
    #[test]
    fn test_rot_1() {
        assert_eq!(caesar("test",1), "uftu");
    }
    #[test]
    fn test_rot_2() {
        assert_eq!(caesar("test",2), "vguv");
    }
    #[test]
    fn test_rot_3() {
        assert_eq!(caesar("test",3), "whvw");
    }
    #[test]
    fn test_rot_4() {
        assert_eq!(caesar("test",4), "xiwx");
    }
    #[test]
    fn test_rot_5() {
        assert_eq!(caesar("test",5), "yjxy");
    }
    #[test]
    fn test_rot_6() {
        assert_eq!(caesar("test",6), "zkyz");
    }
    #[test]
    fn test_rot_7() {
        assert_eq!(caesar("test",7), "alza");
    }
    #[test]
    fn test_rot_8() {
        assert_eq!(caesar("test",8), "bmab");
    }
    #[test]
    fn test_rot_9() {
        assert_eq!(caesar("test",9), "cnbc");
    }
    #[test]
    fn test_rot_10() {
        assert_eq!(caesar("test",10), "docd");
    }
    #[test]
    fn test_rot_11() {
        assert_eq!(caesar("test",11), "epde");
    }
    #[test]
    fn test_rot_12() {
        assert_eq!(caesar("test",12), "fqef");
    }
    #[test]
    fn test_rot_13() {
        assert_eq!(caesar("test",13), "grfg");
    }
    #[test]
    fn test_rot_14() {
        assert_eq!(caesar("test",14), "hsgh");
    }
    #[test]
    fn test_rot_15() {
        assert_eq!(caesar("test",15), "ithi");
    }
    #[test]
    fn test_rot_16() {
        assert_eq!(caesar("test",16), "juij");
    }
    #[test]
    fn test_rot_17() {
        assert_eq!(caesar("test",17), "kvjk");
    }
    #[test]
    fn test_rot_18() {
        assert_eq!(caesar("test",18), "lwkl");
    }
    #[test]
    fn test_rot_19() {
        assert_eq!(caesar("test",19), "mxlm");
    }
    #[test]
    fn test_rot_20() {
        assert_eq!(caesar("test",20), "nymn");
    }
    #[test]
    fn test_rot_21() {
        assert_eq!(caesar("test",21), "ozno");
    }
    #[test]
    fn test_rot_22() {
        assert_eq!(caesar("test",22), "paop");
    }
    #[test]
    fn test_rot_23() {
        assert_eq!(caesar("test",23), "qbpq");
    }
    #[test]
    fn test_rot_24() {
        assert_eq!(caesar("test",24), "rcqr");
    }
    #[test]
    fn test_rot_25() {
        assert_eq!(caesar("test",25), "sdrs");
    }
    #[test]
    fn test_rot_26() {
        assert_eq!(caesar("test",26), "test");
    }
}
