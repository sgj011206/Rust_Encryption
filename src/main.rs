
fn add(a: i32, b: i32) -> i32 {
    a + b 
}

fn main() {
    let result = add(5, 10);
    println!("The result is: {}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3); 
    }
    #[test]
    fn test_main() {
        main(); 
    }
}
