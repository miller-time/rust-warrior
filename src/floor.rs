pub struct Floor {
    pub width: usize,
    pub height: usize,
    pub warrior: (i32, i32),
    pub stairs: (i32, i32),
}

impl Floor {
    pub fn new(warrior: (i32, i32)) -> Floor {
        Floor {
            width: 8,
            height: 1,
            warrior,
            stairs: (7, 0),
        }
    }

    pub fn draw(&self) {
        println!(" {}", "-".repeat(self.width));

        let tiles: Vec<&str> = (0..self.width)
            .map(|x| {
                if (x as i32, 0) == self.warrior {
                    "@"
                } else if (x as i32, 0) == self.stairs {
                    ">"
                } else {
                    " "
                }
            })
            .collect();
        println!("|{}|", tiles.join(""));

        println!(" {}", "-".repeat(self.width));
    }
}
