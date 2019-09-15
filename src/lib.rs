#[macro_use]
extern crate nom;
extern crate failure;

pub mod core;
pub mod uri;
pub mod headers;
pub mod builder;
pub(crate) mod parse;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
