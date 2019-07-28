use std::fmt;

#[derive(Clone, Copy)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub warrior: (usize, usize),
    pub stairs: (usize, usize),
}

impl Level {
    pub fn new(level: usize) -> Level {
        match level {
            1 => Level {
                width: 8,
                height: 1,
                warrior: (0, 0),
                stairs: (7, 0),
            },
            _ => unimplemented!(),
        }
    }

    pub fn move_warrior(&mut self) {
        let (x, y) = self.warrior;
        self.warrior = (x + 1, y);
    }

    pub fn is_complete(&self) -> bool {
        self.warrior == self.stairs
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tiles: Vec<&str> = (0..self.width)
            .map(|x| {
                if (x, 0) == self.warrior {
                    "@"
                } else if (x, 0) == self.stairs {
                    ">"
                } else {
                    " "
                }
            })
            .collect();
        let tiles = format!("|{}|", tiles.join(""));
        write!(
            f,
            " {wall}\n{tiles}\n {wall}",
            wall = "-".repeat(self.width),
            tiles = tiles
        )
    }
}