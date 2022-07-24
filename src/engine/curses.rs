use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::path::Path;

use ncurses::*;

const COMBAT_LOG_PATH: &str = "combat.log";

pub struct Curses {
    pub w: *mut i8,
    pub log: File,
}

impl Curses {
    pub fn new() -> Self {
        initscr();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        let w = newwin(20, 100, 1, 1);

        if Path::new(COMBAT_LOG_PATH).exists() {
            remove_file(COMBAT_LOG_PATH)
                .expect(&format!("failed to delete existing {}", COMBAT_LOG_PATH));
        }

        let log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(COMBAT_LOG_PATH)
            .expect(&format!("failed to create {}", COMBAT_LOG_PATH));
        Curses { w, log }
    }

    pub fn clear(&self) {
        wclear(self.w);
    }

    pub fn println(&mut self, s: &str) {
        waddstr(self.w, s);
        waddch(self.w, '\n' as u32);
        wrefresh(self.w);

        self.log
            .write_all(&s.as_bytes())
            .expect(&format!("failed to write to {}", COMBAT_LOG_PATH));
        self.log
            .write_all(b"\n")
            .expect(&format!("failed to write to {}", COMBAT_LOG_PATH));
    }
}

impl Drop for Curses {
    fn drop(&mut self) {
        endwin();
    }
}
