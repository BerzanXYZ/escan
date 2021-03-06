mod client;
mod response;
mod enums;
mod endpoints;

pub use client::*;
pub use enums::*;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
