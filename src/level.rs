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
        match Level::get(level) {
            Some(level) => level,
            _ => unimplemented!(),
        }
    }

    pub fn exists(level: usize) -> bool {
        Level::get(level).is_some()
    }

    pub fn move_warrior(&mut self) {
        let (x, y) = self.warrior;
        self.warrior = (x + 1, y);
    }

    pub fn is_complete(&self) -> bool {
        self.warrior == self.stairs
    }

    fn get(level: usize) -> Option<Level> {
        match level {
            1 => Some(Level {
                width: 8,
                height: 1,
                warrior: (0, 0),
                stairs: (7, 0),
            }),
            2 => Some(Level {
                width: 8,
                height: 1,
                warrior: (0, 0),
                stairs: (7, 0),
                // TODO: sludge
            }),
            _ => None,
        }
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
