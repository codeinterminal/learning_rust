fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    #[test]
    fn fail_test() {
        println!("I am an output : 2 != 3");
        assert_eq!(2, 3);
    }

    #[test]
    fn success_test() {
        println!("I am a success output : 2 == 3");
        assert_eq!(2, 2);
    }


    #[test]
    fn fail_test_again() -> Result<(), String> {
        println!("I am an output");
        return Err("big fail".into());
    }
}
