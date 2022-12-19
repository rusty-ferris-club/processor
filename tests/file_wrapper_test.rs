use anyhow::Result;
use processor::find_line_start;
use processor::Combiner;
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

impl Combiner<u8> for FileWrapper {
    fn size(&self) -> u64 {
        self.data.get_ref().metadata().unwrap().len()
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

    fn get_current_position(&mut self) -> u64 {
        self.data.stream_position().unwrap()
    }

    fn set_current_position(&mut self, current_position: u64) {
        self.data.seek(SeekFrom::Start(current_position)).unwrap();
        self.current_position = self.get_current_position();
    }

    fn read_line(&mut self) -> String {
        let mut line = String::new();
        self.data.read_line(&mut line).unwrap();
        line.trim_end().to_string()
    }

    fn read_one(&mut self, forward: bool) -> Option<u8> {
        if !forward {
            let new_position = self.get_current_position() - 1;
            self.set_current_position(new_position);
        }
        self.data.by_ref().bytes().next().transpose().unwrap()
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
    fn test_find_line() {
        use super::*;
        struct TestData {
            pub start: u64,
            pub found: u64,
            pub line: String,
        }

        impl TestData {
            fn new(start: u64, found: u64, line: String) -> Self {
                Self { start, found, line }
            }
        }

        for test_data in vec![
            TestData::new(0, 0, "line-1".to_string()),
            TestData::new(3, 0, "line-1".to_string()),
            TestData::new(8, 7, "line-2".to_string()),
            TestData::new(10, 7, "line-2".to_string()),
        ] {
            let mut file = FileWrapper::new("tests_data/test.log".to_string()).unwrap();
            let result = find_line_start(&mut file, test_data.start);
            assert_eq!(result, test_data.found);
            let line = file.read_line();
            assert_eq!(line, test_data.line);
        }
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
