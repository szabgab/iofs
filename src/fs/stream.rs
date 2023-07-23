use crate::io::ConResult;
use crate::io::ConvertBuffer;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub trait FileReadStream {
    fn start_reading(&mut self) -> io::Result<()>;
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>;
    fn read_to_bytes(&mut self) -> io::Result<Vec<u8>>;
    fn read_to_any<B: ConvertBuffer>(&mut self) -> ConResult<B>;
    fn read_to_string(&mut self) -> io::Result<String>;
    fn read_until<B: ConvertBuffer>(&mut self, byte: u8) -> ConResult<B>;
    fn read_line<B: ConvertBuffer>(&mut self) -> ConResult<B>;
    fn lines(&self) -> ConResult<Lines<BufReader<File>>>;
}

pub struct Lines<B> {
    reader: B,
}

impl<B> Lines<B> {
    pub fn new(reader: B) -> Lines<B> {
        Lines { reader }
    }
}
impl<B: io::BufRead> Iterator for Lines<B> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

pub trait WriteBuffer {
    fn as_buf(&self) -> &[u8];
}

impl WriteBuffer for Vec<u8> {
    fn as_buf(&self) -> &[u8] {
        self.as_slice()
    }
}
impl WriteBuffer for String {
    fn as_buf(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl WriteBuffer for &[u8] {
    fn as_buf(&self) -> &[u8] {
        &self
    }
}

pub trait BufferStream {
    fn write_buf<'a>(&'a self) -> Box<dyn WriteBuffer + 'a>;
}

macro_rules! derive_raw {
    (for $($t:ty), +) => {
        $(impl BufferStream for $t {
            fn write_buf(&self) -> Box<dyn WriteBuffer> {
                Box::new(self.to_string())
            }
        })*
    };
}
derive_raw!(
    for i8, i16, i32, isize, i64, i128, u8, u16, u32, usize, u64, u128, f32, f64, char
);

macro_rules! derive_str {
    (for $($t:ty), +) => {
        $(impl BufferStream for $t {
            fn write_buf(&self) -> Box<dyn WriteBuffer + '_> {
                Box::new(self.as_bytes())
            }
        })*
    };
}
derive_str!(for &str, &String, String);

macro_rules! derive_vec {
    (for $($t:ty), +) => {
        $(impl BufferStream for $t {
            fn write_buf(&self) -> Box<dyn WriteBuffer + '_> {
                Box::new(self.as_slice())
            }
        })*
    };
}
derive_vec!(for &Vec<u8>, Vec<u8>);

impl BufferStream for &[u8] {
    fn write_buf<'a>(&'a self) -> Box<dyn WriteBuffer + 'a> {
        Box::new(*self)
    }
}

pub trait FileWriteStream {
    fn start_writing(&mut self) -> io::Result<()>;
    fn write<T: BufferStream>(&mut self, contents: T) -> io::Result<()>;
    fn writeln<T: BufferStream>(&mut self, contents: T) -> io::Result<()>;
    fn overwrite<T: BufferStream>(&mut self, contents: T) -> io::Result<()>;
}
