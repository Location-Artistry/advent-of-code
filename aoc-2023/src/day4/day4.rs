use std::{fs, char, env};

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    fn read_file(file_path: &str) -> String {
        println!("In file {}", file_path); 
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        contents
    }
    fn lines_to_vec_str(contents: &String) -> Vec<&str> {
        let all_lines: Vec<&str> = contents
                .lines()
                .collect();
        return all_lines
    }

    fn compare_numbers(line: &str) -> u32 {
        let mut matches: u32 = 0;
        for(i, cards) in line.split(":").collect::<Vec<&str>>().iter().enumerate() {
            let card = cards.split("|").collect::<Vec<&str>>();
            let numbers = card.iter().collect::<Vec<_>>();
            match i == 1 {
                false => (),
                true => {
                    let win = numbers[0].trim()
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap_or(0))
                        .collect::<Vec<_>>();
                    let yours = numbers[1].trim()
                        .split_whitespace()
                        .map(|x| x.trim().parse::<u32>().unwrap_or(0))
                        .collect::<Vec<_>>();
                    println!("{:?}", win );
                    println!("{:?}", yours );
                    for y in yours.iter() {
                        matches = matches + win.clone()
                                 .into_iter()
                                 .filter(|x| x == y)
                                 .collect::<Vec<u32>>().len() as u32;
                    }
                    println!("{}", matches);
                }
            }
        }
        return match matches {
            0 => 0, 1 => 1, 2 => 2, 3 => 4, 4 => 8, 5 => 16, 
            6 => 32, 7 => 64, 8 => 128, 9 => 256, 10 => 512, _ => 0
        }
    }

    let contents = read_file("./day4/input");
    let lines_vec = lines_to_vec_str(&contents);
    let mut sum: u32 = 0; 
    for line in lines_vec.iter() { 
        println!("{}", sum);
        sum = sum + compare_numbers(line);
    }
    println!("SUM OF CARD SCORES: {}", sum);
}


