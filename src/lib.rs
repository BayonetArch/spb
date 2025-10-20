use simple_term_attr::{DisplayAttribute, StyleAttributes};
use std::io::{Write, stdout};
use std::sync::{LazyLock, Mutex};

static TERM_SIZE: LazyLock<(u16, u16)> = LazyLock::new(|| DisplayAttribute::get_term_size());
static BAR: Mutex<String> = Mutex::new(String::new());

pub fn initial_bar_setup() {
    println!();
    DisplayAttribute::save_cursor_pos();

    DisplayAttribute::set_scrollable_region(0, TERM_SIZE.1 - 1);

    DisplayAttribute::restore_cursor_pos();
    DisplayAttribute::move_cursor_x_lines_up(1);
    let _ = DisplayAttribute::hide_cursor();
}

pub fn restore_bar_setup() {
    DisplayAttribute::save_cursor_pos();

    DisplayAttribute::set_scrollable_region(0, TERM_SIZE.1);
    DisplayAttribute::move_cursor(TERM_SIZE.1, 2);
    DisplayAttribute::clear_line();

    DisplayAttribute::restore_cursor_pos();
    let _ = DisplayAttribute::show_cursor();
}

#[allow(unused)]
pub fn progress_bar(total_task: usize, current_task: usize) {
    let perc_done = (current_task as f32 / total_task as f32) * 100.0;

    let bar_width = TERM_SIZE.0 as usize / 2;
    let filled = (perc_done / 100.0 * bar_width as f32) as usize;

    let mut bar = BAR.lock().unwrap();

    *bar = "[".to_string();
    bar.push_str(&"=".repeat(filled));
    bar.push_str(&" ".repeat(bar_width - filled));
    bar.push(']');

    DisplayAttribute::save_cursor_pos();

    DisplayAttribute::move_cursor(TERM_SIZE.1, 2);
    DisplayAttribute::clear_line();

    print!("{}:  {bar}  {perc_done:.1}%\r", "Progress".blue_bold());
    stdout().flush();

    DisplayAttribute::restore_cursor_pos();
}
