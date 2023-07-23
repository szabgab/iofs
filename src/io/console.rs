use std::fmt::Display;
use std::io::{stdout, Write, stdin, StdinLock, BufRead, self};
use std::process::{Command, ExitStatus};
use super::ConResult;
use super::convert::ConvertBuffer;


pub struct Console<'a> {
    inner: StdinLock<'a>,
}

impl<'a> Console<'a> {
    pub fn new() -> Console<'a> {
        Console { inner: stdin().lock() }
    }
    pub fn lock(&mut self) -> &mut StdinLock<'a> {
        &mut self.inner
    }

    pub fn read_to_i32(&mut self) -> i32 {
        self.input(None).unwrap_or(-1)
    }

    pub fn read_to_f64(&mut self) -> f64 {
        self.input(None).unwrap_or(0.0)
    }

    pub fn read_until(&mut self, byte: u8) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.inner.read_until(byte, &mut buf)?;
        
        buf.pop();
        if buf.ends_with(&[b'\r']) {
            buf.pop();
        }
        Ok(buf)
    }

    pub fn input<B: ConvertBuffer>(&mut self, text: Option<&str>) -> ConResult<B> {
        if let Some(contents) = text {
            flush_output(contents);
        }
        let buf = self.read_until(b'\n')?;
        B::from_buf(buf)
    }
 

    pub fn print<T: Display>(contents: T, fg: Option<usize>, bg: Option<usize>) {
        print(contents, fg, bg);
        stdout().flush().unwrap_or_default()
    }
    pub fn println<T: Display>(contents: T, fg: Option<usize>, bg: Option<usize>) {
        print(contents, fg, bg);
        println!()
    }
    pub fn clear() -> io::Result<ExitStatus> {
        if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/C")
                .arg("cls").status()
        } else {
            Command::new("sh").arg("-c")
                .arg("clear").status()
        }
    }

    pub fn hide_cursor() -> io::Result<()> {
        print!("\x1b[?;25;l");
        stdout().flush()
    }
    /// Show the Cursor in terminal
    pub fn show_cursor() -> io::Result<()> {
        print!("\x1b[?;25;h");
        stdout().flush()
    }

    pub fn set_cursor_position(x: usize, y: usize) -> io::Result<()> {
        print!("\x1b[{};{}H", y + 1, x + 1);
        stdout().flush()
    }


}


fn print<T: Display>(contents: T, fg: Option<usize>, bg: Option<usize>) {
    let reset = "\x1b[0m";
    let str = match fg {
        Some(fg) => {
            format!("\x1b[38;5;{fg}m{}{reset}", contents)
        }
        _ => contents.to_string(),
    };
    let str = match bg {
        Some(bg) => format!("\x1b[48;5;{bg}m{str}{reset}"),
        _ => str
    };
    print!("{}", str);
}



fn flush_output(contents: impl Display) {
    print!("{}", contents);
    stdout().flush().unwrap_or_default()
}