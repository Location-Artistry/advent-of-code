fn main() {
    let time_dist: Vec<u64> = vec![47707566, 282107911471062];
    let mut margin_totals: Vec<u32> = Vec::new(); 
        let (t0, d0, mut current_margin) = (time_dist[0], time_dist[1], 0);
        for tr in 1..t0 {
            let dr = tr * (t0 - tr);
            match dr > d0 {
                true => current_margin +=1,
                false => (),
            }
        }
        margin_totals.push(current_margin);
        println!("{:?}", margin_totals);
    let sum = margin_totals.into_iter().reduce(|a, b| a * b);
    println!("The answer for day6 part2: {:?}", sum.unwrap());
}
