use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::{Error, ErrorKind, Read};
use std::mem::size_of;

#[derive(Debug, Eq, PartialEq)]
pub enum LogLevel {
    Verbose = 0x02,
    Debug = 0x03,
    Info = 0x04,
    Warn = 0x05,
    Error = 0x06,
    Fatal = 0x07,
}

impl TryFrom<u8> for LogLevel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0x02 => Ok(Self::Verbose),
            0x03 => Ok(Self::Debug),
            0x04 => Ok(Self::Info),
            0x05 => Ok(Self::Warn),
            0x06 => Ok(Self::Error),
            0x07 => Ok(Self::Fatal),
            _ => Err(()),
        }
    }
}

/// A digest of the 'struct logger_entry' structs adb logcat may produce. See Android's
/// system/core/liblog/include/log/log_read.h for details, particulary commit 441054aa1e0 which
/// consolidates the logger_entry_v? versions.
#[derive(Debug, Eq, PartialEq)]
pub struct LoggerEntry {
    pub pid: i32,
    pub tid: u32,
    pub sec: u32,
    pub nsec: u32,
    pub level: LogLevel,
    pub tag: String,
    pub text: String,
}

pub fn parse_logger_entry<R>(src: &mut R) -> Result<LoggerEntry, Error>
where
    R: Read,
{
    let payload_size = src.read_u16::<LittleEndian>()? as usize;
    let header_size = match src.read_u16::<LittleEndian>()? {
        0 => 5 * size_of::<u32>(),
        x => x as usize,
    };
    let pid = src.read_i32::<LittleEndian>()?;
    let tid = src.read_u32::<LittleEndian>()?;
    let sec = src.read_u32::<LittleEndian>()?;
    let nsec = src.read_u32::<LittleEndian>()?;

    // skip to get to the payload: ignore fields that may or may not exist depending on what
    // version of struct logger_entry this is
    for _ in 5..(header_size / size_of::<u32>()) {
        src.read_u32::<LittleEndian>()?;
    }

    // payload contains 'level:u8 tag:[u8] \0 text:[u8] \0'
    let mut payload = vec![0u8; payload_size];
    src.read_exact(&mut payload)?;

    let level = LogLevel::try_from(payload[0]).map_err(|_| ErrorKind::InvalidData)?;
    let pos = payload
        .iter()
        .position(|&x| x == 0)
        .ok_or(ErrorKind::InvalidData)?;
    let tag = &payload[1..pos];
    let text = &payload[pos + 1..payload.len() - 1];

    Ok(LoggerEntry {
        pid,
        tid,
        sec,
        nsec,
        level,
        tag: String::from_utf8_lossy(tag).into_owned(),
        text: String::from_utf8_lossy(text).into_owned(),
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::logcat::{parse_logger_entry, LogLevel, LoggerEntry};

    #[test]
    fn parse_valid_data() {
        let bytes =
            include_bytes!("../tests/data/pixel-2-android-p-developer-preview-easter-egg.bin");
        let mut cursor = Cursor::new(&bytes[..]);

        let entry = parse_logger_entry(&mut cursor).unwrap();
        assert_eq!(
            entry,
            LoggerEntry {
                pid: 819,
                tid: 933,
                sec: 1522927821,
                nsec: 332446033,
                level: LogLevel::Info,
                tag: "CHRE".to_string(),
                text: "@ 54.638: [AR_CHRE] still: 100\n".to_string(),
            }
        );

        let entry = parse_logger_entry(&mut cursor).unwrap();
        assert_eq!(
            entry,
            LoggerEntry {
                pid: 1179,
                tid: 1277,
                sec: 1522927823,
                nsec: 95501053,
                level: LogLevel::Info,
                tag: "ActivityManager".to_string(),
                text: "Killing 3849:com.google.android.settings.intelligence/u0a51 (adj 906): empty #17".to_string(),
            }
        );
    }
}
