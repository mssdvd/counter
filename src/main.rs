fn main() {
    let mut pressed_chars: indexmap::IndexMap<char, u32> = indexmap::IndexMap::new();

    ncurses::initscr();
    ncurses::noecho();

    ncurses::addstr("Counter by mssdvd\n");

    loop {
        for (char, times) in &pressed_chars {
            ncurses::addstr(&format!(
                "Char: `{}` pressed {} times\n",
                char.escape_default(),
                times
            ));
        }

        let ch = ncurses::getch();
        let ch = char::from_u32(ch as u32).expect("Invalid char");

        let count = pressed_chars.entry(ch as char).or_insert(0);
        *count += 1;
        ncurses::clear();
    }
}
