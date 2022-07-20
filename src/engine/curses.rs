use ncurses::*;

pub struct Curses {
    pub w: *mut i8,
}

impl Curses {
    pub fn new() -> Self {
        initscr();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        let w = newwin(20, 100, 1, 1);
        Curses { w }
    }

    pub fn clear(&self) {
        wclear(self.w);
    }

    pub fn println(&self, s: &str) {
        waddstr(self.w, s);
        waddch(self.w, '\n' as u32);
        wrefresh(self.w);
    }
}

impl Drop for Curses {
    fn drop(&mut self) {
        endwin();
    }
}
