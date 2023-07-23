use iofs::{
    fs::stream::{FileReadStream, FileWriteStream},
    prelude::{FileInfo, Find},
};

fn main() {
    let mut file = FileInfo::open("he");
    let mut txt = FileInfo::open_smart("content").unwrap();
    txt.start_writing().unwrap();
    file.start_reading().unwrap();
    while let Ok(buf) = file.read_line::<String>() {
        let log: Vec<_> = buf.trim().split("\t").collect();
        txt.writeln(format!("\"{}\" => \"{}\", ", log[0], log[1])).unwrap();
        txt.writeln(format!("\"{}\" => \"{}\", ", log[2], log[3])).unwrap();
    }
}
