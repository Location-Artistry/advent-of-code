# day6

Goal is to go further than the current record holder in specified time
Toy boats with buttons, button charges the boat

Time:      7  15   30
Distance:  9  40  200

Race 1:
dist = 0x7 = 0
dist = 1x6 = 6
dist = 2x5 = 10*
dist = 3x4 = 12*
dist = 4x3 = 112*
dist = 5x2 = 10*
dist = 1x6 = 6

Race 2:
dist = 3x12 = 36 
dist = 4x11 = 44* 
dist = 5x10 = 50* 
dist = 6x9 = 54* 
dist = 7x8 = 56* 
dist = 8x7 = 56*
dist = 6x9 = 54*
dist = 5x10 = 50* 
dist = 4x11 = 44*
dist = 3x12 = 36

Time:      7  15   30
Distance:  9  40  200

time_dist: Vec<Vec<u32>> = [[7,9], [15,40], [30,200]]
let margin_totals: Vec<u32>
for td in time_dist {
    t0 = td.[0]
    d0 = td.[1]
    current_margin = 0
    for tr in range 1..t0 {
        dr = tr x (t0 - tr) // 1 x 6 = 6
        match dr > d0 {
            true => current_margin += 1
            false => ()
        }
    }
    margin_totals.push(current_margin)
}
margin_totals.reduce(|x, acc| x + acc)


