use super::fd::FileDir;
use super::stream::{FileReadStream, FileWriteStream};
use super::Attributes;
use super::DFiles;
use crate::fs::pathstr::FileString;
use crate::fs::pbuilder::PathBuilder;
use crate::fs::stream::{BufferStream, Lines};
use crate::io::convert::ConvertBuffer;
use crate::io::{ConResult, ConvertError};
use std::fmt::Debug;
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Result, Write};

pub struct FileInfo {
    inner: PathBuilder,
    stream: Stream,
}

unsafe impl Send for FileInfo {}
unsafe impl Sync for FileInfo {}
impl Clone for FileInfo {
    fn clone(&self) -> Self {
        FileInfo {
            inner: self.inner.clone(),
            stream: Stream::None,
        }
    }
}

#[cfg(feature = "web_warp")]
impl warp::Reply for FileInfo {
    fn into_response(self) -> warp::reply::Response {
        use url::form_urlencoded::byte_serialize;
        use warp::http::HeaderValue;
        use warp::hyper::header;
        match std::fs::read(self.full_name()) {
            Ok(buf) => {
                let mut res = warp::reply::Response::new(buf.into());
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(self.content_type()),
                );
                let encoded_filename = byte_serialize(self.name().as_bytes()).collect::<String>();

                let content_disposition = format!("attachment; filename={}", encoded_filename);
                res.headers_mut().insert(
                    warp::hyper::header::CONTENT_DISPOSITION,
                    HeaderValue::from_str(&content_disposition).unwrap(),
                );
                res.headers_mut().insert(
                    header::ACCESS_CONTROL_EXPOSE_HEADERS,
                    HeaderValue::from_static("Content-Disposition"),
                );
                res
            }
            _ => warp::hyper::StatusCode::NOT_FOUND.into_response(),
        }
    }
}

impl From<File> for FileInfo {
    fn from(value: File) -> Self {
        let value = value.to_string();
        FileInfo {
            inner: unsafe { PathBuilder::from_uncheck(value) },
            stream: Stream::None,
        }
    }
}

enum Stream {
    Write(File),
    Read(BufReader<File>),
    None,
}

impl Debug for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileInfo")
            .field("full_name", &self.inner.full_name())
            .finish()
    }
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl DFiles for FileInfo {
    fn is_exist(&self) -> bool {
        match self.metadata() {
            Ok(data) => data.is_file(),
            _ => false,
        }
    }

    fn attributes(&self) -> Attributes {
        if self.is_exist() {
            Attributes::File
        } else {
            Attributes::None
        }
    }

    fn builder(&self) -> &PathBuilder {
        self.inner.borrow()
    }

    unsafe fn mut_builder(&mut self) -> &mut PathBuilder {
        self.inner.mut_borrow()
    }

    fn copy_new(&self, path: &str) -> Result<()> {
        let builder = PathBuilder::from(path);
        match builder.is_exist() {
            false => {
                fs::copy(self.full_name(), builder.full_name())?;
                Ok(())
            }
            _ => Err(already_exists_err()),
        }
    }

    fn move_new(&mut self, path: &str) -> Result<()> {
        let builder = PathBuilder::from(path);
        match builder.is_exist() {
            false => {
                fs::copy(self.full_name(), builder.full_name())?;
                fs::remove_file(self.full_name())?;
                *self = unsafe { FileInfo::open_uncheck(builder.full_name()) };
                Ok(())
            }
            _ => Err(already_exists_err()),
        }
    }
    fn size_bytes(&self) -> u64 {
        match self.metadata() {
            Ok(data) => data.len(),
            _ => 0,
        }
    }
}

impl FileInfo {
    pub fn open<P: AsRef<str>>(value: P) -> Self {
        FileInfo {
            inner: PathBuilder::from(value),
            stream: Stream::None,
        }
    }
    pub unsafe fn open_uncheck<P: AsRef<str>>(path: P) -> FileInfo {
        FileInfo {
            inner: PathBuilder::from_uncheck(path.as_ref()),
            stream: Stream::None,
        }
    }

    #[cfg(feature = "web_warp")]
    pub fn content_type(&self) -> &'static str {
        match self.extension() {
            ".*" => "application/octet-stream",
            ".001" => "application/x-001",
            ".301" => "application/x-301",
            ".323" => "text/h323",
            ".906" => "application/x-906",
            ".907" => "drawing/907",
            ".a11" => "application/x-a11",
            ".acp" => "audio/x-mei-aac",
            ".ai" => "application/postscript",
            ".aif" => "audio/aiff",
            ".aifc" => "audio/aiff",
            ".aiff" => "audio/aiff",
            ".anv" => "application/x-anv",
            ".asa" => "text/asa",
            ".asf" => "video/x-ms-asf",
            ".asp" => "text/asp",
            ".asx" => "video/x-ms-asf",
            ".au" => "audio/basic",
            ".avi" => "video/avi",
            ".awf" => "application/vnd.adobe.workflow",
            ".biz" => "text/xml",
            ".bmp" => "application/x-bmp",
            ".bot" => "application/x-bot",
            ".c4t" => "application/x-c4t",
            ".c90" => "application/x-c90",
            ".cal" => "application/x-cals",
            ".cat" => "application/vnd.ms-pki.seccat",
            ".cdf" => "application/x-netcdf",
            ".cdr" => "application/x-cdr",
            ".cel" => "application/x-cel",
            ".cer" => "application/x-x509-ca-cert",
            ".cg4" => "application/x-g4",
            ".cgm" => "application/x-cgm",
            ".cit" => "application/x-cit",
            ".class" => "java/*",
            ".cml" => "text/xml",
            ".cmp" => "application/x-cmp",
            ".cmx" => "application/x-cmx",
            ".cot" => "application/x-cot",
            ".crl" => "application/pkix-crl",
            ".crt" => "application/x-x509-ca-cert",
            ".csi" => "application/x-csi",
            ".css" => "text/css",
            ".cut" => "application/x-cut",
            ".dbf" => "application/x-dbf",
            ".dbm" => "application/x-dbm",
            ".dbx" => "application/x-dbx",
            ".dcd" => "text/xml",
            ".dcx" => "application/x-dcx",
            ".der" => "application/x-x509-ca-cert",
            ".dgn" => "application/x-dgn",
            ".dib" => "application/x-dib",
            ".dll" => "application/x-msdownload",
            ".doc" => "application/msword",
            ".dot" => "application/msword",
            ".drw" => "application/x-drw",
            ".dtd" => "text/xml",
            ".dwf" => "application/x-dwf",
            ".dwg" => "application/x-dwg",
            ".dxb" => "application/x-dxb",
            ".dxf" => "application/x-dxf",
            ".edn" => "application/vnd.adobe.edn",
            ".emf" => "application/x-emf",
            ".eml" => "message/rfc822",
            ".ent" => "text/xml",
            ".epi" => "application/x-epi",
            ".eps" => "application/postscript",
            ".etd" => "application/x-ebx",
            ".exe" => "application/x-msdownload",
            ".fax" => "image/fax",
            ".fdf" => "application/vnd.fdf",
            ".fif" => "application/fractals",
            ".fo" => "text/xml",
            ".frm" => "application/x-frm",
            ".g4" => "application/x-g4",
            ".gbr" => "application/x-gbr",
            "." => "application/x-",
            ".gif" => "image/gif",
            ".gl2" => "application/x-gl2",
            ".gp4" => "application/x-gp4",
            ".hgl" => "application/x-hgl",
            ".hmr" => "application/x-hmr",
            ".hpg" => "application/x-hpgl",
            ".hpl" => "application/x-hpl",
            ".hqx" => "application/mac-binhex40",
            ".hrf" => "application/x-hrf",
            ".hta" => "application/hta",
            ".htc" => "text/x-component",
            ".htm" => "text/html",
            ".html" => "text/html",
            ".htt" => "text/webviewhtml",
            ".htx" => "text/html",
            ".icb" => "application/x-icb",
            ".ico" => "image/x-icon",
            ".iff" => "application/x-iff",
            ".ig4" => "application/x-g4",
            ".igs" => "application/x-igs",
            ".iii" => "application/x-iphone",
            ".img" => "application/x-img",
            ".ins" => "application/x-internet-signup",
            ".isp" => "application/x-internet-signup",
            ".IVF" => "video/x-ivf",
            ".java" => "java/*",
            ".jfif" => "image/jpeg",
            ".jpe" => "image/jpeg",
            ".jpeg" => "image/jpeg",
            ".jpg" => "image/jpeg",
            ".js" => "application/x-javascript",
            ".jsp" => "text/html",
            ".json" => "application/json",
            ".la1" => "audio/x-liquid-file",
            ".lar" => "application/x-laplayer-reg",
            ".latex" => "application/x-latex",
            ".lavs" => "audio/x-liquid-secure",
            ".lbm" => "application/x-lbm",
            ".lmsff" => "audio/x-la-lms",
            ".ls" => "application/x-javascript",
            ".ltr" => "application/x-ltr",
            ".m1v" => "video/x-mpeg",
            ".m2v" => "video/x-mpeg",
            ".m3u" => "audio/mpegurl",
            ".m4e" => "video/mpeg4",
            ".mac" => "application/x-mac",
            ".man" => "application/x-troff-man",
            ".math" => "text/xml",
            ".mdb" => "application/x-mdb",
            ".mfp" => "application/x-shockwave-flash",
            ".mht" => "message/rfc822",
            ".mhtml" => "message/rfc822",
            ".mi" => "application/x-mi",
            ".mid" => "audio/mid",
            ".midi" => "audio/mid",
            ".mil" => "application/x-mil",
            ".mml" => "text/xml",
            ".mnd" => "audio/x-musicnet-download",
            ".mns" => "audio/x-musicnet-stream",
            ".mocha" => "application/x-javascript",
            ".movie" => "video/x-sgi-movie",
            ".mp1" => "audio/mp1",
            ".mp2" => "audio/mp2",
            ".mp2v" => "video/mpeg",
            ".mp3" => "audio/mp3",
            ".mp4" => "video/mpeg4",
            ".mpa" => "video/x-mpg",
            ".mpd" => "application/vnd.ms-project",
            ".mpe" => "video/x-mpeg",
            ".mpeg" => "video/mpg",
            ".mpg" => "video/mpg",
            ".mpga" => "audio/rn-mpeg",
            ".mpp" => "application/vnd.ms-project",
            ".mps" => "video/x-mpeg",
            ".mpt" => "application/vnd.ms-project",
            ".mpv" => "video/mpg",
            ".mpv2" => "video/mpeg",
            ".mpw" => "application/vnd.ms-project",
            ".mpx" => "application/vnd.ms-project",
            ".mtx" => "text/xml",
            ".mxp" => "application/x-mmxp",
            ".net" => "image/pnetvue",
            ".nrf" => "application/x-nrf",
            ".nws" => "message/rfc822",
            ".odc" => "text/x-ms-odc",
            ".out" => "application/x-out",
            ".p10" => "application/pkcs10",
            ".p12" => "application/x-pkcs12",
            ".p7b" => "application/x-pkcs7-certificates",
            ".p7c" => "application/pkcs7-mime",
            ".p7m" => "application/pkcs7-mime",
            ".p7r" => "application/x-pkcs7-certreqresp",
            ".p7s" => "application/pkcs7-signature",
            ".pc5" => "application/x-pc5",
            ".pci" => "application/x-pci",
            ".pcl" => "application/x-pcl",
            ".pcx" => "application/x-pcx",
            ".pdf" => "application/pdf",
            ".pdx" => "application/vnd.adobe.pdx",
            ".pfx" => "application/x-pkcs12",
            ".pgl" => "application/x-pgl",
            ".pic" => "application/x-pic",
            ".pko" => "application/vnd.ms-pki.pko",
            ".pl" => "application/x-perl",
            ".plg" => "text/html",
            ".pls" => "audio/scpls",
            ".plt" => "application/x-plt",
            ".png" => "image/png",
            ".pot" => "application/vnd.ms-powerpoint",
            ".ppa" => "application/vnd.ms-powerpoint",
            ".ppm" => "application/x-ppm",
            ".pps" => "application/vnd.ms-powerpoint",
            ".ppt" => "application/vnd.ms-powerpoint",
            ".pr" => "application/x-pr",
            ".prf" => "application/pics-rules",
            ".prn" => "application/x-prn",
            ".prt" => "application/x-prt",
            ".ps" => "application/x-ps",
            ".ptn" => "application/x-ptn",
            ".pwz" => "application/vnd.ms-powerpoint",
            ".r3t" => "text/vnd.rn-realtext3d",
            ".ra" => "audio/vnd.rn-realaudio",
            ".ram" => "audio/x-pn-realaudio",
            ".ras" => "application/x-ras",
            ".rat" => "application/rat-file",
            ".rdf" => "text/xml",
            ".rec" => "application/vnd.rn-recording",
            ".red" => "application/x-red",
            ".rgb" => "application/x-rgb",
            ".rjs" => "application/vnd.rn-realsystem-rjs",
            ".rjt" => "application/vnd.rn-realsystem-rjt",
            ".rlc" => "application/x-rlc",
            ".rle" => "application/x-rle",
            ".rm" => "application/vnd.rn-realmedia",
            ".rmf" => "application/vnd.adobe.rmf",
            ".rmi" => "audio/mid",
            ".rmj" => "application/vnd.rn-realsystem-rmj",
            ".rmm" => "audio/x-pn-realaudio",
            ".rmp" => "application/vnd.rn-rn_music_package",
            ".rms" => "application/vnd.rn-realmedia-secure",
            ".rmvb" => "application/vnd.rn-realmedia-vbr",
            ".rmx" => "application/vnd.rn-realsystem-rmx",
            ".rnx" => "application/vnd.rn-realplayer",
            ".rp" => "image/vnd.rn-realpix",
            ".rpm" => "audio/x-pn-realaudio-plugin",
            ".rsml" => "application/vnd.rn-rsml",
            ".rt" => "text/vnd.rn-realtext",
            ".rtf" => "application/msword",
            ".rv" => "video/vnd.rn-realvideo",
            ".sam" => "application/x-sam",
            ".sat" => "application/x-sat",
            ".sdp" => "application/sdp",
            ".sdw" => "application/x-sdw",
            ".sit" => "application/x-stuffit",
            ".slb" => "application/x-slb",
            ".sld" => "application/x-sld",
            ".slk" => "drawing/x-slk",
            ".smi" => "application/smil",
            ".smil" => "application/smil",
            ".smk" => "application/x-smk",
            ".snd" => "audio/basic",
            ".sol" => "text/plain",
            ".sor" => "text/plain",
            ".spc" => "application/x-pkcs7-certificates",
            ".spl" => "application/futuresplash",
            ".spp" => "text/xml",
            ".ssm" => "application/streamingmedia",
            ".sst" => "application/vnd.ms-pki.certstore",
            ".stl" => "application/vnd.ms-pki.stl",
            ".stm" => "text/html",
            ".sty" => "application/x-sty",
            ".svg" => "text/xml",
            ".swf" => "application/x-shockwave-flash",
            ".tdf" => "application/x-tdf",
            ".tg4" => "application/x-tg4",
            ".tga" => "application/x-tga",
            ".tif" => "image/tiff",
            ".tiff" => "image/tiff",
            ".tld" => "text/xml",
            ".top" => "drawing/x-top",
            ".torrent" => "application/x-bittorrent",
            ".tsd" => "text/xml",
            ".txt" => "text/plain",
            ".uin" => "application/x-icq",
            ".uls" => "text/iuls",
            ".vcf" => "text/x-vcard",
            ".vda" => "application/x-vda",
            ".vdx" => "application/vnd.visio",
            ".vml" => "text/xml",
            ".vpg" => "application/x-vpeg005",
            ".vsd" => "application/vnd.visio",
            ".vss" => "application/vnd.visio",
            ".vst" => "application/vnd.visio",
            ".vsw" => "application/vnd.visio",
            ".vsx" => "application/vnd.visio",
            ".vtx" => "application/vnd.visio",
            ".vxml" => "text/xml",
            ".wav" => "audio/wav",
            ".wax" => "audio/x-ms-wax",
            ".wb1" => "application/x-wb1",
            ".wb2" => "application/x-wb2",
            ".wb3" => "application/x-wb3",
            ".wbmp" => "image/vnd.wap.wbmp",
            ".wiz" => "application/msword",
            ".wk3" => "application/x-wk3",
            ".wk4" => "application/x-wk4",
            ".wkq" => "application/x-wkq",
            ".wks" => "application/x-wks",
            ".wm" => "video/x-ms-wm",
            ".wma" => "audio/x-ms-wma",
            ".wmd" => "application/x-ms-wmd",
            ".wmf" => "application/x-wmf",
            ".wml" => "text/vnd.wap.wml",
            ".wmv" => "video/x-ms-wmv",
            ".wmx" => "video/x-ms-wmx",
            ".wmz" => "application/x-ms-wmz",
            ".wp6" => "application/x-wp6",
            ".wpd" => "application/x-wpd",
            ".wpg" => "application/x-wpg",
            ".wpl" => "application/vnd.ms-wpl",
            ".wq1" => "application/x-wq1",
            ".wr1" => "application/x-wr1",
            ".wri" => "application/x-wri",
            ".wrk" => "application/x-wrk",
            ".ws" => "application/x-ws",
            ".ws2" => "application/x-ws",
            ".wsc" => "text/scriptlet",
            ".wsdl" => "text/xml",
            ".wvx" => "video/x-ms-wvx",
            ".xdp" => "application/vnd.adobe.xdp",
            ".xdr" => "text/xml",
            ".xfd" => "application/vnd.adobe.xfd",
            ".xfdf" => "application/vnd.adobe.xfdf",
            ".xhtml" => "text/html",
            ".xls" => "application/x-xls",
            ".xlw" => "application/x-xlw",
            ".xml" => "text/xml",
            ".xpl" => "audio/scpls",
            ".xq" => "text/xml",
            ".xql" => "text/xml",
            ".xquery" => "text/xml",
            ".xsd" => "text/xml",
            ".xsl" => "text/xml",
            ".xslt" => "text/xml",
            ".xwd" => "application/x-xwd",
            ".x_b" => "application/x-x_b",
            ".sis" => "application/vnd.symbian.install",
            ".sisx" => "application/vnd.symbian.install",
            ".x_t" => "application/x-x_t",
            ".ipa" => "application/vnd.iphone",
            ".apk" => "application/vnd.android.package-archive",
            ".xap" => "application/x-silverlight-app",
            _ => "application/others",
        }
    }
    pub fn open_smart<P: AsRef<str>>(path: P) -> Result<FileInfo> {
        let f = match File::open(path.as_ref()) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let path = PathBuilder::from(path);
                DirBuilder::new().recursive(true).create(path.parent())?;
                File::create(path.full_name())?
            }
            Err(error) => return Err(error),
        };
        Ok(FileInfo::from(f))
    }

    pub fn is_eq(&self, other: &Self) -> bool {
        fn is_eq(f: &FileInfo, other: &FileInfo) -> Result<bool> {
            if f.size_bytes() == other.size_bytes() {
                let mut f = ReadFile::new(f.file()?);
                let mut other = ReadFile::new(other.file()?);
                loop {
                    let Some(f_read) = f.read() else { return Ok(true)};
                    let Some(other_read) = other.read() else { return Ok(false)};
                    return match f_read == other_read {
                        true => continue,
                        _ => Ok(false),
                    };
                }
            } else {
                Ok(false)
            }
        }
        is_eq(self, other).unwrap_or(false)
    }

    pub fn del(self) -> Result<()> {
        fs::remove_file(self.full_name())
    }

    pub fn extension(&self) -> &str {
        self.inner.extension()
    }
    pub fn set_extension(&mut self, extension: &str) -> Result<()> {
        let pre_path = self.inner.clone();
        self.inner.set_extension(extension);
        fs::rename(pre_path.full_name(), self.full_name())
    }
    fn file(&self) -> Result<File> {
        File::open(self.full_name())
    }
    pub fn to_file_dir(self) -> FileDir {
        FileDir::from(self)
    }

    fn error(&self, is_read: bool) -> ConvertError {
        if self.is_exist() {
            con_permission(is_read)
        } else {
            con_not_found()
        }
    }
}

impl FileWriteStream for FileInfo {
    fn start_writing(&mut self) -> Result<()> {
        let f = append_file(self.full_name())?;
        self.stream = Stream::Write(f);
        Ok(())
    }

    fn write<T: BufferStream>(&mut self, contents: T) -> Result<()> {
        match &mut self.stream {
            Stream::Write(f) => {
                let write_buf = contents.write_buf();
                f.write_all(write_buf.as_buf())
            }
            _ => Err(self.error(true).into()),
        }
    }

    fn writeln<T: BufferStream>(&mut self, contents: T) -> Result<()> {
        match &mut self.stream {
            Stream::Write(f) => {
                f.write_all(contents.write_buf().as_buf())?;
                f.write_all(&[b'\n'])
            }
            _ => Err(self.error(true).into()),
        }
    }

    fn overwrite<T: BufferStream>(&mut self, contents: T) -> Result<()> {
        let mut f = File::create(self.full_name())?;
        f.write_all(contents.write_buf().as_buf())?;
        self.stream = Stream::Write(f);
        Ok(())
    }
}

impl FileReadStream for FileInfo {
    fn start_reading(&mut self) -> Result<()> {
        let f = File::open(self.full_name())?;
        self.stream = Stream::Read(BufReader::new(f));
        Ok(())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        match &mut self.stream {
            Stream::Read(reader) => reader.read_exact(buf),
            _ => Err(self.error(false).into()),
        }
    }
    fn lines(&self) -> ConResult<Lines<BufReader<File>>> {
        match self.stream {
            Stream::Read(_) => match File::open(self.full_name()) {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    Ok(Lines::new(reader))
                }
                _ => Err(self.error(false)),
            },
            _ => Err(self.error(false)),
        }
    }
    fn read_to_string(&mut self) -> std::io::Result<String> {
        match &mut self.stream {
            Stream::Read(reader) => {
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;
                Ok(buf)
            }
            _ => Err(self.error(false).into()),
        }
    }
    fn read_to_bytes(&mut self) -> std::io::Result<Vec<u8>> {
        match &mut self.stream {
            Stream::Read(reader) => {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                Ok(buf)
            }
            _ => Err(self.error(false).into()),
        }
    }

    fn read_to_any<B: ConvertBuffer>(&mut self) -> ConResult<B> {
        B::from_buf(self.read_to_bytes()?)
    }

    fn read_until<B: ConvertBuffer>(&mut self, byte: u8) -> ConResult<B> {
        match &mut self.stream {
            Stream::Read(reader) => {
                let mut buf = Vec::new();
                reader.read_until(byte, &mut buf)?;
                if buf.is_empty() {
                    return Err(ConvertError::Empty);
                }
                if buf.ends_with(&[byte]) {
                    buf.pop();
                    if byte == b'\n' && buf.ends_with(&[b'\r']) {
                        buf.pop();
                    }
                }
                B::from_buf(buf)
            }
            _ => Err(self.error(false).into()),
        }
    }

    fn read_line<B: ConvertBuffer>(&mut self) -> ConResult<B> {
        self.read_until(b'\n')
    }
}
pub fn read_first_line(path: &str) -> Result<String> {
    let f = File::open(path)?;
    let mut buf = String::new();
    let mut reader = BufReader::new(f);
    reader.read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

fn con_permission(is_read: bool) -> ConvertError {
    let error = if is_read {
        "Maybe you should call this function 'start_writing()'."
    } else {
        "Maybe you should call this function 'start_reading()'."
    };
    ConvertError::IoError(Error::new(ErrorKind::PermissionDenied, error))
}
fn con_not_found() -> ConvertError {
    ConvertError::IoError(not_found_err())
}

fn already_exists_err() -> Error {
    Error::new(ErrorKind::AlreadyExists, "The file already exists!")
}

fn append_file(path: &str) -> Result<File> {
    match OpenOptions::new().append(true).open(path) {
        Ok(f) => Ok(f),
        _ => File::open(path),
    }
}

struct ReadFile {
    file: File,
    end: bool,
    count: u64,
    pos: u64,
}

impl ReadFile {
    pub fn new(file: File) -> ReadFile {
        let len = match file.metadata() {
            Ok(data) => data.len(),
            _ => 0,
        };
        let count = len / 131_072;
        ReadFile {
            file,
            end: false,
            count,
            pos: 0,
        }
    }

    pub fn read(&mut self) -> Option<Vec<u8>> {
        if !self.end && self.count > self.pos {
            let mut arr = [0; 131_072];
            self.pos += 1;
            match self.file.read_exact(&mut arr) {
                Ok(_) => Some(arr.to_vec()),
                _ => None,
            }
        } else if !self.end {
            self.end = true;
            let mut vec = Vec::new();
            match self.file.read_to_end(&mut vec) {
                Ok(_) => Some(vec),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn not_found_err() -> Error {
    Error::new(ErrorKind::NotFound, "The specified file cannot be found!")
}
