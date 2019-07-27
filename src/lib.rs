pub mod game;
pub mod level;
pub mod templates;
pub mod ui;
pub mod warrior;

pub use warrior::Warrior;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
