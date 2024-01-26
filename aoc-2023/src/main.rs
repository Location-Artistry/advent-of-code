use std::fs;

fn main() {

    fn reduce_vec(v0: &Vec<i32>) -> Vec<i32> {
        let mut v1: Vec<i32> = Vec::new();
        for(i, z) in v0.iter().enumerate() {
            match i != v0.len()-1 {
                true => v1.push(v0[i+1] - z), 
                false => () 
            }
        }
        v1
    }

    fn recurse_reduce(v0: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut cont: bool = true;
        let mut first: bool = true;
        let mut v1: Vec<i32> = Vec::new();
        let mut vec_list: Vec<Vec<i32>> = Vec::new();
        vec_list.push(v0.clone());
        while cont == true {
            v1 = match first == true {
                true => reduce_vec(v0), 
                false => reduce_vec(&v1)
            };
            vec_list.push(v1.clone());
            match v1.clone().into_iter().reduce(|acc, b| acc + b).unwrap() == 0 {
                true => cont = false,
                false =>  ()
            }
            first = false;
        }
        vec_list
    }

    fn find_next_num (vec_list: &Vec<Vec<i32>>) -> i32 {
        let last_nums = vec_list.iter().map(|x|*x.last().unwrap()).reduce(|acc, b| acc+b);
        last_nums.unwrap()
    }

    fn read_file(file_path: &str) -> String {
        println!("In file {}", file_path); 
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        contents
    }

    fn lines_to_vec_num(contents: &String) -> Vec<Vec<i32>> {
        let all_lines: Vec<&str> = contents.lines().collect();
        let mut all_lines_nums: Vec<Vec<i32>> = Vec::new();
        for line in all_lines.iter() {
            let input = line
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().unwrap_or(0))
                .collect::<Vec<_>>();
            all_lines_nums.push(input);
        }
        return all_lines_nums
    }

    fn find_all_next_num(lines_vec: Vec<Vec<i32>>) -> i32 {
        lines_vec
            .iter()
            .map(|lin| recurse_reduce(&lin))
            .collect::<Vec<_>>()
            .iter()
            .map(|vec_list| find_next_num(vec_list))
            .sum()
    }

    // program workflow
    let contents = read_file("./src/day9/input");
    let lines_vec = lines_to_vec_num(&contents);
    println!("The answer for day9 part1: {}", find_all_next_num(lines_vec)); 
}
