use anyhow::Result;
use processor::state::traits::{find_line_start, Combiner};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

pub struct FileWrapper {
    pub current_position: u64,
    pub start_marker: u8,
    pub end_marker: u8,
    pub data: BufReader<File>,
}

impl FileWrapper {
    pub fn new(file: String) -> Result<Self> {
        let data = BufReader::new(File::open(file)?);

        Ok(Self {
            current_position: 0,
            start_marker: b'\n',
            end_marker: b'\n',
            data,
        })
    }
}

impl Combiner for FileWrapper {
    fn get_current_position(&mut self) -> u64 {
        self.data.stream_position().unwrap()
    }

    fn set_current_position(&mut self, current_position: u64) {
        self.data.seek(SeekFrom::Start(current_position)).unwrap();
        self.current_position = self.get_current_position();
    }

    fn set_start_marker(&mut self, start: u8) {
        self.start_marker = start;
    }

    fn get_start_marker(&self) -> u8 {
        self.start_marker
    }

    fn set_end_marker(&mut self, end: u8) {
        self.end_marker = end;
    }

    fn get_end_marker(&self) -> u8 {
        self.end_marker
    }

    fn read_exact(&mut self, size: i64, buf: &mut [u8]) {
        if size < 0 {
            let new_position = self.get_current_position() - size.abs() as u64;
            self.set_current_position(new_position);
        }
        self.data.read_exact(buf).unwrap();
    }

    fn read_line(&mut self) -> String {
        let mut line = String::new();
        self.data.read_line(&mut line).unwrap();
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_find_line_start_0_0() {
        use super::*;
        let mut file = FileWrapper::new("tests_data/test.log".to_string()).unwrap();
        let result = find_line_start(&mut file, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_find_line_start_3_0() {
        use super::*;
        let mut file = FileWrapper::new("tests_data/test.log".to_string()).unwrap();
        let result = find_line_start(&mut file, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_find_line_start_8_0() {
        use super::*;
        let mut file = FileWrapper::new("tests_data/test.log".to_string()).unwrap();
        let result = find_line_start(&mut file, 8);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_find_line_start_10_0() {
        use super::*;
        let mut file = FileWrapper::new("tests_data/test.log".to_string()).unwrap();
        let result = find_line_start(&mut file, 10);
        assert_eq!(result, 7);
    }
}
