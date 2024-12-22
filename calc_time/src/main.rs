use std::time::SystemTime;

fn main() {
    let arr: Vec<i32> = (1..=1000).collect();

    let start = SystemTime::now();

    let result = sum_of_add(&arr);

    let end = SystemTime::now();
    println!("Time taken: {:?} for {:?}", end.duration_since(start).unwrap(),
        result
    );
}

fn sum_of_add(input: &[i32]) -> i32 {
    input.iter().fold(0, |acc, &x| acc + x + 1)
}