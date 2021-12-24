fn main() {
    println!("Hello, world!");
}

fn plus(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::plus;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, plus(a, b));
    }
}