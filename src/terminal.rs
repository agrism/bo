use crate::Position;
use std::cmp;
use std::fmt;
use std::io::{self, stdout, Write};
use termion::color;
use termion::event::{Event, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, ToAlternateScreen, ToMainScreen};

#[derive(Debug)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: AlternateScreen<MouseTerminal<RawTerminal<std::io::Stdout>>>,
}

impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Terminal")
            .field("size", &self.size)
            .finish()
    }
}

impl Terminal {
    /// # Errors
    ///
    /// will return an error if the terminal size can't be acquired
    /// or if the stdout cannot be put into raw mode.
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                height: size.1.saturating_sub(2), // to leave space for the status and message bars
                width: size.0,
            },
            _stdout: AlternateScreen::from(MouseTerminal::from(stdout().into_raw_mode()?)),
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    /// # Errors
    ///
    /// Returns an error if stdout can't be flushed
    pub fn flush() -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }

    /// # Errors
    ///
    /// Returns an error if a event can't be read
    pub fn read_event() -> Result<Event, std::io::Error> {
        loop {
            let opt_key = io::stdin().lock().events().next();
            // at that point, event is a Result<Event, Error>, as the Option was unwrapped
            if let Some(event) = opt_key {
                return event;
            }
        }
    }

    pub fn set_cursor_position(&mut self, position: &Position) {
        let Position {
            mut x,
            mut y,
            mut x_offset,
        } = position;
        // hiding the fact that the terminal position is 1-based, while preventing an overflow
        x_offset += if x_offset > 0 { 1 } else { 0 };
        x = x.saturating_add(1);
        x = cmp::min(x.saturating_add(x_offset.into()), self.size.width.into());
        y = y.saturating_add(1);
        y = cmp::min(y, self.size.height.into());
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }

    #[must_use]
    pub fn get_cursor_index_from_mouse_event(mouse_event: MouseEvent, x_offset: u8) -> Position {
        if let MouseEvent::Press(_, x, y) = mouse_event {
            let offset_adjustment: u8 = if x_offset > 0 {
                x_offset.saturating_add(1)
            } else {
                0
            };
            Position {
                x: x.saturating_sub(1)
                    .saturating_sub(u16::from(offset_adjustment)) as usize,
                y: y.saturating_sub(1) as usize,
                x_offset,
            }
        } else {
            Position::top_left()
        }
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn to_alternate_screen() {
        print!("{}", ToAlternateScreen);
    }

    pub fn to_main_screen() {
        print!("{}", ToMainScreen);
    }

    pub fn clear_all() {
        print!("{}", termion::clear::All);
    }
}
