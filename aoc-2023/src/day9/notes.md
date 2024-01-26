# day9part1
## Another Flippin Floatin Island

Sand Instability Sensor

0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45

Make prediction of the next value in each history

Line1: increase by 3 

0   3   6   9  12  15  18
  3   3   3   3   3   3 
    0   0   0   0   0

1   3   6  10  15  21  28
  2   3   4   5   6   7
    1   1   1   1   1
      0   0   0   0 

let values: Vec<u32> = [1, 3, 6, 10, 15, 21]
values_diff = map values.(|x| x[i-1])
for i, z in values_diff.iter() {
    z[i+2] - z[i+1]
```rust
fn reduce_vec(v0: Vec<u32>) {
    let mut v1: new Vec;
    for i, x in enumerate(v.iter()) {
        match i != v0.len() {
            true => v1.push(x[i+1] - x[i]) // [3, 3, 3, 3, 3]
            false => () 
        }
    } 
}
let v0: Vec<u32> = [0, 3,  6,  9, 12, 15];
v1 = reduce_vec(v0);
v1 = [3, 3, 3, 3, 3]
v2 = reduce_vec(v1);
v2 = [0, 0, 0, 0];
reduce_test(v2)
fn reduce_test(v: Vec<u32>) {

let sum = .into_iter().reduce(|a, b| a * b);

data_vecs[[0,3,6,9,12,15], [3,3,3,3,3], [0,0,0,0]];
// does not look correct!
// v2.push(0)
// last = data_vecs[data_vecs.len()-1]
// last.reduce(|acc=last[0], b| acc - b) === 0 ?
//     then inc = last[0]
last.push(0)

vp = data_vecs[0]

fn check_inc (v: Vec<u32>) {
    let l = v.len();
    let inc: u32 = 0;
    match (v[l]-v[l-1]) == (v[l-1]-v[l-2]) {
        true => inc = v[l],
        false => ()
    }
    inc
}
    l_minus = data_vecs[data_vecs.len()-1]
l = last.len()




```

