mod doc;
mod domain;
mod handler;
mod route;

pub use doc::*;
pub use domain::*;
pub use handler::*;
pub use route::*;

#[cfg(test)]
mod test {
    #[test]
    fn dummy_test() {
        assert!(true);
    }
}
