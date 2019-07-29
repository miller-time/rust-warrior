use crate::engine::components::WarriorComponent;

pub struct Warrior<'a> {
    component: &'a mut WarriorComponent,
    performed_action: bool,
}

impl<'a> Warrior<'a> {
    pub fn new(component: &'a mut WarriorComponent) -> Warrior<'a> {
        Warrior {
            component,
            performed_action: false,
        }
    }

    pub fn walk(&mut self) {
        if self.performed_action {
            println!("Warrior already performed action!");
            return;
        }
        println!("Warrior moves forward");
        let (x, y) = self.component.position;
        self.component.position = (x + 1, y);
        self.performed_action = true;
    }
}
