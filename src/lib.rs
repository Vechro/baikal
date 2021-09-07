#![feature(once_cell)]

pub mod binary_prefix;
pub mod conversion_table;
mod error;

pub mod prelude {
    pub use crate::error::Error;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
