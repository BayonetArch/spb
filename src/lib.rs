use simple_term_attr::*;
use std::io::{self, Write, stdout};
use std::sync::{LazyLock, Mutex};

static TERM_SIZE: LazyLock<(u16, u16)> = LazyLock::new(|| get_terminal_size());
static BAR: Mutex<String> = Mutex::new(String::new());

pub fn initial_bar_setup() -> io::Result<()> {
    println!();

    save_cursor_pos()?;

    set_scrollable_region(0, TERM_SIZE.1 - 1)?;

    restore_cursor_pos()?;
    move_cursor_up(1)?;
    hide_cursor()?;
    Ok(())
}

pub fn restore_bar_setup() -> io::Result<()> {
    save_cursor_pos()?;

    set_scrollable_region(0, TERM_SIZE.1)?;
    move_cursor(TERM_SIZE.1, 2)?;
    clear_line()?;

    restore_cursor_pos()?;
    show_cursor()?;
    Ok(())
}

#[allow(unused)]
pub fn progress_bar(total_task: usize, current_task: usize) {
    let perc_done = (current_task as f32 / total_task as f32) * 100.0;

    let bar_width = TERM_SIZE.0 as usize / 2;
    let filled = (perc_done / 100.0 * bar_width as f32) as usize;

    let mut bar = BAR.lock().unwrap();

    *bar = "[".to_string();

    bar.push_str(&"#".repeat(filled));
    bar.push_str(&".".repeat(bar_width - filled));
    bar.push(']');

    save_cursor_pos();

    move_cursor(TERM_SIZE.1, 2);
    clear_line();

    print!(
        "{}: [{current_task}/{total_task}] {bar} {perc_done:.1}%\r",
        "Progress".green_bold()
    );
    stdout().flush();

    restore_cursor_pos();
}
