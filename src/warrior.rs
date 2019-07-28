/// The protagonist of rust-warrior!
#[derive(Default)]
pub struct Warrior;

impl Warrior {
    pub fn walk(&self) {
        println!("walking forward!");
    }
}
