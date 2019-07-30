use crate::actions::Action;

pub struct Warrior {
    pub path_clear: bool,
    pub action: Option<Action>,
}

impl Warrior {
    pub fn new(path_clear: bool) -> Warrior {
        Warrior {
            path_clear,
            action: None,
        }
    }

    pub fn walk(&mut self) {
        if self.action.is_some() {
            println!("Warrior already performed action!");
            return;
        }

        self.action = Some(Action::Walk);
    }

    pub fn path_clear(&self) -> bool {
        self.path_clear
    }

    pub fn attack(&mut self) {
        if self.action.is_some() {
            println!("Warrior already performed action!");
            return;
        }

        self.action = Some(Action::Attack);
    }
}
