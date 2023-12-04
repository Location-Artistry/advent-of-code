use std::fs;

fn main() {
    fn read_file(file_path: &str) {
        println!("In file {}", file_path); 
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let mut all_lines : Vec<String> = Vec::new();
        for line in contents.lines() {
            let mut digits = String::new();
            for c in line.chars() {
                let char_string = String::from(c);
                match c.is_numeric() {
                    true => digits.push_str(&char_string),
                    false => (),
                };
            }
            match digits.len() {
                2 => all_lines.push(digits),
                1 => { 
                    digits.push_str(&digits.clone());
                    all_lines.push(digits)  
                }
                _ => {
                    let mut new_digits = String::from(digits.chars().next().unwrap());
                    new_digits.push_str(&String::from(digits.chars().last().unwrap().clone()));
                    all_lines.push(new_digits);
                }
            }
        }
        let mut all_lines_int : Vec<u32> = Vec::new();
        for num in all_lines {
            all_lines_int.push(num.parse::<u32>().unwrap())
        }
        println!("{}", all_lines_int.iter().fold(0, |acc, elem| acc + elem));
    }
    read_file("./src/day1/input")
}
