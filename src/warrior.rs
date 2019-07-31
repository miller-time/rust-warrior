use crate::actions::Action;

pub struct Warrior {
    path_clear: bool,
    health: i32,
    pub action: Option<Action>,
}

impl Warrior {
    pub fn new(path_clear: bool, health: i32) -> Warrior {
        Warrior {
            path_clear,
            health,
            action: None,
        }
    }

    pub fn walk(&mut self) {
        self.perform(Action::Walk);
    }

    pub fn path_clear(&self) -> bool {
        self.path_clear
    }

    pub fn attack(&mut self) {
        self.perform(Action::Attack);
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn rest(&mut self) {
        self.perform(Action::Rest);
    }

    fn perform(&mut self, action: Action) {
        if self.action.is_some() {
            println!("Warrior already performed action!");
            return;
        }

        self.action = Some(action);
    }
}
