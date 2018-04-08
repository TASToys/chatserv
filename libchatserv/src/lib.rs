#[macro_use]
extern crate serde_derive;
extern crate nats;

pub mod bot;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

//    pub fn
}
