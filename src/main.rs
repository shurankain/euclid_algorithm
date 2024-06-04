fn main() {
    let args: Vec<String> = std::env::args().collect();

    let a = args[1].parse::<i64>().unwrap();
    let b = args[2].parse::<i64>().unwrap();

    println!("Result: {}", euqlid(a, b));
}

fn euqlid(mut a: i64, mut b: i64) -> i64 {
    let mut res: i64 = 0;
    while (b != 0) {
        res = a % b;
        a = b;
        b = res;
    }
    return a;
}



