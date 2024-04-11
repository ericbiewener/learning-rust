use std::collections::HashMap;

fn get_median(ints: &Vec<i32>) -> i32 {
    let idx = ints.len() / 2;
    ints[idx]
}

fn get_mode(ints: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    let mut largest_k: i32 = 0;
    let mut largest_v: i32 = 0;
    
    for int in ints {
        let v = map.entry(int).or_insert(0);
        *v += 1;
        if *v > largest_v {
            largest_k = *int;
            largest_v = *v;
        }
    }

    println!("{:?}", map);
    return largest_k;
}

fn main() {
    let mut ints = vec![7, 1, 6, 2, 3, 9, 9, 6, 9];
    ints.sort();
    let median = get_median(&ints);
    let mode = get_mode(&ints);
    println!("median: {median}");
    println!("mode: {mode}");
}
