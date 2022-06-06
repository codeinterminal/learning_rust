fn main() {
    let (a, b) = return_pair(29);
    for i in 1..=2 {
        println!("{} -> a {}, b {}", i, a, b);
    }
}

fn return_pair(x: i32) -> (i32, i64) {
    let z : i64 = (x + 1).into();
    let y = x - 1;
    (y, z)
}
