use std::io;

fn main() {
    let a : i8 = 120;

    let mut in_str = String::new();
    io::stdin().read_line(&mut in_str)
        .expect("cannot readline");

    let mut b : i8 = in_str.trim().parse().expect("integer");
    b += a;
    println!("result {}", b);
}
