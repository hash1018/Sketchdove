pub mod message;
pub mod user;

pub const IP_ADDRESS: &str = "::1";
pub const PORT: &str = "8080";

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
