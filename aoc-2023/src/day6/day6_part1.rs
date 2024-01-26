fn main() {
    let time_dist = vec![[47,282],[70,1079],[75,1147],[66,1062]];
    let mut margin_totals: Vec<u32> = Vec::new(); 
        for(i, td) in time_dist.iter().enumerate() {
            let (t0, d0, mut current_margin) = (td[0], td[1], 0);
            for tr in 1..t0 {
                let dr = tr * (t0 - tr);
                match dr > d0 {
                    true => current_margin +=1,
                    false => (),
                }
            }
            margin_totals.push(current_margin);
            println!("{:?}", margin_totals);
        }
        let sum = margin_totals.into_iter().reduce(|a, b| a * b);
        println!("The answer for day6 part1: {:?}", sum.unwrap());
}
