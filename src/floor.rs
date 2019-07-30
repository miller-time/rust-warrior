#[derive(Clone, Copy, Debug)]
pub struct Floor {
    pub width: usize,
    pub height: usize,
    pub warrior: (i32, i32),
    pub stairs: (i32, i32),
    pub sludge: Option<(i32, i32)>,
}

impl Floor {
    pub fn load(level: usize) -> Floor {
        match Floor::get(level) {
            Some(level) => level,
            None => unimplemented!(),
        }
    }

    pub fn exists(level: usize) -> bool {
        Floor::get(level).is_some()
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
                    match self.sludge {
                        Some(sludge) if (x as i32, 0) == sludge => "s",
                        _ => " ",
                    }
                }
            })
            .collect();
        println!("|{}|", tiles.join(""));

        println!(" {}", "-".repeat(self.width));
    }

    fn get(level: usize) -> Option<Floor> {
        match level {
            1 => Some(Floor { ..Floor::default() }),
            2 => Some(Floor {
                sludge: Some((4, 0)),
                ..Floor::default()
            }),
            _ => None,
        }
    }

    fn default() -> Floor {
        Floor {
            width: 8,
            height: 1,
            warrior: (0, 0),
            stairs: (7, 0),
            sludge: None,
        }
    }
}
