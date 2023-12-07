use std::fs;

fn main() {
    fn read_file(file_path: &str) {
        println!("In file {}", file_path); 
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        #[derive(Debug)]
        struct Grab {
            red: u32,
            blue: u32,
            green: u32,
        }
        let bag: Grab = Grab {red: 12, green: 13, blue: 14};
        let mut id_sum: u32 = 0;
        for(index, line) in contents.lines().enumerate() {
            let mut game_possible: bool = true;
            let mut index = index as u32;
            index += 1;
            println!("{}", line);
            // Game
            for(i, grab) in line.split(";").enumerate() {
                let mut counter: Grab = Grab {red: 0, green: 0, blue: 0};
                if i == 0 {
                    // when in first grab which has index
                    for(z, cubes) in grab.split(":").enumerate() {
                    // iterate through cubes
                        if z != 0 {
                            for cube in cubes.split(",") {
                                let v: Vec<&str> = cube.split(' ').collect();
                                match v[2] {
                                    "red" => counter.red = counter.red + v[1].parse::<u32>().unwrap(),
                                    "blue" => counter.blue = counter.blue + v[1].parse::<u32>().unwrap(), 
                                    "green" => counter.green = counter.green + v[1].parse::<u32>().unwrap(), 
                                    _ => println!("did not match {:?}", v)
                                }
                            }
                        }
                    }
                } 
                else {
                    for cube in grab.split(",") {
                        let v: Vec<&str> = cube.split(' ').collect();
                        match v[2] {
                            "red" => counter.red = counter.red + v[1].parse::<u32>().unwrap(),
                            "blue" => counter.blue = counter.blue + v[1].parse::<u32>().unwrap(), 
                            "green" => counter.green = counter.green + v[1].parse::<u32>().unwrap(), 
                            _ => println!("did not match {:?}", v)
                        }
                    }
                }
                if counter.red > bag.red { game_possible = false }
                else if counter.green > bag.green { game_possible = false }
                else if counter.blue > bag.blue { game_possible = false } 
            } // end of grab for loop
            if game_possible == true { id_sum = id_sum + index }

            println!("index: {} - id_sum: {}", index, id_sum)
        } // end of lines and games for loop
    }
    
    read_file("./src/day2/input")
}

