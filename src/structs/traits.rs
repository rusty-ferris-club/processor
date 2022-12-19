pub trait Combiner<Data> {
    fn size(&self) -> u64;
    fn set_start_marker(&mut self, start: Data);
    fn get_start_marker(&self) -> Data;
    fn set_end_marker(&mut self, end: Data);
    fn get_end_marker(&self) -> Data;
    fn get_current_position(&mut self) -> u64;
    fn set_current_position(&mut self, current_position: u64);
    fn read_line(&mut self) -> String;
    fn read_one(&mut self, forward: bool) -> Option<Data>;
    // fn calculate_chunks(&mut self, cpus: u64) -> Vec<[u64; 2]>;
}

pub fn find_line_start<Source, Data>(data: &mut Source, start_offset: u64) -> u64
where
    Source: Combiner<Data>,
    Data: PartialEq,
{
    let mut buf: Option<Data> = None;
    data.set_current_position(start_offset);
    while data.get_current_position() > 0 {
        buf = data.read_one(false);
        if buf.unwrap() == data.get_start_marker() {
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

pub fn calculate_chunks<Source, Data>(data: &mut Source, cpus: u64) -> Vec<[u64; 2]>
where
    Source: Combiner<Data>,
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

pub fn process_line<Source, Data>(proccessor: &dyn FnMut(&[Data]) -> bool) {
    todo!("Need to implement");
}
