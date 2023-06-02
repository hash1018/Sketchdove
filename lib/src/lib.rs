pub mod message;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub const PPP: u32 = 10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
