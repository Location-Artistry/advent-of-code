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

    fn get_char_matrix(lines_vec: Vec<&str>) -> Vec<Vec<char>> {
        let mut y: Vec<Vec<char>> = Vec::new();
        for line in lines_vec.iter() {
            let x: Vec<_> = line.chars().collect();
            y.push(x);
        }
        y
    }

    fn find_adj_symbols(matrix: Vec<Vec<char>>) -> i32 {
        let mut part_num_sum: i32 = 0;
        let symbols = vec!['!','@','#','$','%','^','&','*','(',')','+','-', '/', '=', '`','\\','[',']','|'];
        for(y, line) in matrix.iter().enumerate() {
            let mut part_num = String::new();
            println!("{:?}, {:?}", line, y);
            for(i, &x1) in line.iter().enumerate() {
                // println!("{} - {}", i, line.len());
                let check_number = if x1.is_numeric()==true && i==(line.len()-1) { false } else if x1.is_numeric()==true { true } else { false }; 
                match check_number {
                    true => part_num.push(x1),
                    false => {
                        if x1.is_numeric()==true && i==(line.len()-1) { part_num.push(x1) };
                        let mut part_added: bool = false;
                        if part_num.len() > 0 {
                            for part_x_pos in (i-part_num.len())..i {
                                let prev_x = part_x_pos.checked_sub(1).unwrap_or(0); 
                                let prev_y = y.checked_sub(1).unwrap_or(0);
                                let next_y = match y == matrix.len()-1 {true => 0, false => 2};
                                let next_x = match part_x_pos == line.len() {true => 0, false => 2};
                                // let next_x = match part_x_pos == line.len() {true => 0, false => 2};
                                for y2 in (prev_y)..(y+next_y) {
                                    for x2 in (prev_x)..(part_x_pos+next_x) {
                                        for s in symbols.iter() {
                                            match &matrix[y2][x2] == s {
                                                true => {
                                                    part_added = true;
                                                },
                                                false => ()
                                            }
                                        }
                                    }
                                }
                            } 
                        part_num_sum = if part_added == true { part_num_sum + part_num.parse::<i32>().unwrap() } else { part_num_sum };
                        println!("{} - {} - {}", part_added, part_num, part_num_sum);
                        }
                        part_num = String::new();
                    }
                }
            }
        }
        part_num_sum
    } 


    let contents = read_file("./day3/input");
    let lines_vec = lines_to_vec_str(&contents);
    let matrix = get_char_matrix(lines_vec);
    let find_symbols = find_adj_symbols(matrix);

    println!("SUM OF ALL PART NUMBERS = {}", find_symbols);
}
