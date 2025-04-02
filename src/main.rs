mod question1;
mod question2;
mod question3;
mod question4;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(1 + 1, 2);
    }
}
