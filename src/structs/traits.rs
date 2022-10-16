pub trait Combiner {
    fn size(&self) -> u64;
    fn set_start_marker(&mut self, start: u8);
    fn get_start_marker(&self) -> u8;
    fn set_end_marker(&mut self, end: u8);
    fn get_end_marker(&self) -> u8;
    fn get_current_position(&mut self) -> u64;
    fn set_current_position(&mut self, current_position: u64);
    fn read_line(&mut self) -> String;
    /// Reads from current_position to current_position + size
    /// and updates the current_position to current_position + size
    fn read_exact(&mut self, size: i64, buf: &mut [u8]);
    fn calculate_chunks(&mut self, cpus: u64) -> Vec<[u64; 2]>;
}

pub fn find_line_start<T>(data: &mut T, start_offset: u64) -> u64
where
    T: Combiner,
{
    let mut buf: &mut [u8] = &mut [0];
    data.set_current_position(start_offset);
    while data.get_current_position() > 0 {
        data.read_exact(-1, buf);
        if buf[0] == data.get_start_marker() {
            break;
        }
        if data.get_current_position() == 0 {
            break;
        }
        let new_position = data.get_current_position() - 1;
        data.set_current_position(new_position);
    }

    data.get_current_position()
}

pub fn calculate_chunks<T>(data: &mut T, cpus: u64) -> Vec<[u64; 2]>
where
    T: Combiner,
{
    let size = data.size();
    let chunk = size / cpus;
    let mut start = 0;
    let mut end = start + chunk;
    let mut reminder = size;
    let mut chunks: Vec<[u64; 2]> = Vec::new();

    while reminder > 0 {
        chunks.push([start, end]);
        start += chunk;
        if start + chunk >= size {
            end = size;
        } else {
            end += chunk;
        }
        reminder = reminder.saturating_sub(chunk);
    }
    chunks
}

pub fn process_line<T>(proccessor: &dyn FnMut(&[u8]) -> bool) {
    println!("Processing line");
}
