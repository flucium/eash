pub mod prompt;
use std::borrow::Cow;
use std::io;
use std::io::Write;
use std::process::exit;

pub struct Terminal {
    buffer: Vec<u8>,
    buffer_index: usize,
    prompt: String,
    origin_termios: libc::termios,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            buffer_index: 0,

            prompt: String::new(),
            origin_termios: termios(),
        }
    }

    pub fn prompt(&mut self, string: String) {
        self.prompt = string
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        self.set_raw_mode();

        self.init_buffer()?;

        let mut stdout = io::BufWriter::new(io::stdout().lock());

        loop {
            stdout.flush()?;

            if let Some(char) = getch() {
                match char {
                    0 => continue,
                    3 => {
                        self.unset_raw_mode();
                        exit(0)
                    }

                    10 => break,

                    27 => {
                        if getch().unwrap_or(27) != 91 {
                            continue;
                        }

                        match getch().unwrap_or(91) {
                            //up
                            65 => {}

                            //down
                            66 => {}

                            //right
                            67 => {
                                if self.buffer_index < self.buffer.len() {
                                    self.buffer_index += 1;
                                    stdout.write_all(
                                        format!("{}", Cursor::Right.get_esc_code()).as_bytes(),
                                    )?;
                                }
                            }

                            //left
                            68 => {
                                if self.buffer_index > 0 {
                                    stdout.write_all(
                                        format!("{}", Cursor::Left.get_esc_code()).as_bytes(),
                                    )?;
                                    self.buffer_index -= 1;
                                }
                            }
                            _ => continue,
                        }
                    }

                    127 => {
                        if self.buffer_index <= 0 {
                            continue;
                        }

                        self.buffer_index -= 1;

                        for i in 0..self.buffer.len() {
                            if i != 0 {
                                stdout.write_all(
                                    format!("{}", Cursor::Backspace.get_esc_code()).as_bytes(),
                                )?;
                            }
                        }

                        stdout.write_all(
                            format!("\r{}{}", self.prompt, String::from_utf8_lossy(&self.buffer))
                                .as_bytes(),
                        )?;

                        self.buffer.remove(self.buffer_index);

                        stdout.write_all(
                            format!("{}", Cursor::Backspace.get_esc_code()).as_bytes(),
                        )?;
                        stdout.write_all(
                            format!(
                                "\r{}{}",
                                self.prompt,
                                String::from_utf8_lossy(&self.buffer).to_string()
                            )
                            .as_bytes(),
                        )?;

                        if self.buffer_index < self.buffer.len() {
                            let move_position = self.prompt.len() + self.buffer_index - 1;
                            stdout.write_all(
                                format!("{}", Cursor::Move(move_position).get_esc_code())
                                    .as_bytes(),
                            )?;
                        }
                    }

                    _ => {
                        self.buffer.insert(self.buffer_index, char);

                        self.buffer_index += 1;

                        for i in 0..self.buffer.len() {
                            if i != 0 {
                                stdout.write_all(
                                    format!("{}", Cursor::Backspace.get_esc_code()).as_bytes(),
                                )?;
                            }
                        }

                        stdout.write_all(
                            format!("\r{}{}", self.prompt, String::from_utf8_lossy(&self.buffer))
                                .as_bytes(),
                        )?;

                        if self.buffer_index < self.buffer.len() {
                            let move_position = self.prompt.len() + self.buffer_index;

                            stdout.write_all(
                                format!("{}", Cursor::Move(move_position).get_esc_code())
                                    .as_bytes(),
                            )?;
                        }
                    }
                }
            }
        }

        self.unset_raw_mode();

        stdout.write_all(b"\n")?;

        Ok(self.get_utf8_str().to_string())
    }

    fn get_utf8_str(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.buffer)
    }

    fn init_buffer(&mut self) -> io::Result<()> {
        let mut stdout = io::BufWriter::new(io::stdout().lock());

        self.buffer.clear();

        self.buffer_index = 0;

        if self.buffer_index <= self.buffer.len() {
            let move_position = self.prompt.len() + 1;
            stdout.write_all(
                format!(
                    "\r{}{}",
                    self.prompt,
                    Cursor::Move(move_position).get_esc_code()
                )
                .as_bytes(),
            )?;
        }

        Ok(())
    }

    fn set_raw_mode(&mut self) {
        unsafe { libc::tcgetattr(0, &mut self.origin_termios) };

        let mut raw = self.origin_termios;

        raw.c_lflag = raw.c_lflag & !(libc::ICANON | libc::ECHO | libc::IEXTEN | libc::ISIG);
        // raw.c_lflag = raw.c_lflag & !(libc::ICANON | libc::ECHO );
        raw.c_cc[libc::VTIME] = 0;

        raw.c_cc[libc::VMIN] = 1;

        unsafe {
            libc::tcsetattr(0, 0, &raw);
            libc::fcntl(0, libc::F_SETFL);
        }
    }

    fn unset_raw_mode(&mut self) {
        unsafe {
            libc::tcsetattr(0, 0, &self.origin_termios);
        }
    }
}

fn getch() -> Option<u8> {
    let code = [0; 1];

    let n = unsafe { libc::read(0, code.as_ptr() as *mut libc::c_void, 1) };

    if n <= 0 {
        return None;
    }

    Some(code[0])
}

#[cfg(target_os = "macos")]
fn termios() -> libc::termios {
    libc::termios {
        c_cc: [0u8; 20],
        c_ispeed: 0,
        c_ospeed: 0,
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
    }
}

#[cfg(target_os = "linux")]
fn termios() -> libc::termios {
    libc::termios {
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
    }
}

enum Cursor {
    Move(usize),
    Backspace,
    Left,
    Right,
    // ClearLine,
}

impl Cursor {
    fn get_esc_code(&self) -> String {
        return match &self {
            Cursor::Move(position) => format!("\x1b[{position}G"),
            Cursor::Backspace => format!("\x08{}", " "),
            Cursor::Left => format!("\x1b[1D"),
            Cursor::Right => format!("\x1b[1C"),
            // Cursor::ClearLine => format!("\x1b[2K"),
        };
    }
}
