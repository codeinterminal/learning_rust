use std::collections::HashMap;



fn main() {
    let mut foo = HashMap::new();

    foo.insert("A".to_string(), 1);
    foo.insert("B".to_string(), 2);
    foo.insert("C".to_string(), 3);
    foo.insert("D".to_string(), 4);

    let key = "X".to_string();
    let value = 203;
    // aqui no hy key ni value
    for (key, value) in foo {
        println!("k {} - v {}", key, value);
    }
    // aqui no hay
    // println!("foo {:?}", foo);
    println!("key {} value {}", key, value);
}
