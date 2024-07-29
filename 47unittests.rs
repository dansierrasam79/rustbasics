
// Open a new project as a Rust library and copy-paste the code below
pub fn prod(x: i32, y: i32) -> i32 {
    x * y
}

// This is not how multiplication is done, 
// Its purpose is to fail in this example.
#[allow(dead_code)]
fn second_prod(x: i32, y: i32) -> i32 {
    x % y
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer scope.
    use super::*;

    #[test]
    fn test_prod() {
        // This test will pass
        assert_eq!(prod(1, 2), 2);
    }

    #[test]
    fn test_prod2() {
        // This assert would fire and test will fail.
        assert_eq!(second_prod(1, 2), 2);
    }
}


