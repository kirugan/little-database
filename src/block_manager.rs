use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::io::SeekFrom;
use std::io::Read;

pub struct BlockManager {
    // todo change type?
    page_size: u64,
    file: File,
}

impl BlockManager {
    // todo move to package level function instead?
    pub fn open(filename: &str, page_size: u64) -> Result<Self, std::io::Error> {
        let file = File::create(filename)?;

        Ok(Self{
            page_size,
            file,
        })
    }

    pub fn read_block(&mut self, i: u64) -> Result<Vec<u8>, std::io::Error> {
        // todo check boundaries
        let mut vec: Vec<u8> = Vec::with_capacity(self.page_size as usize);

        self.file.seek(SeekFrom::Start(i * self.page_size))?;
        self.file.read_exact(vec.as_mut_slice())?;

        return Ok(vec);
    }

    pub fn write_block(&mut self, i: u64, buff: &[u8]) -> Result<(), std::io::Error> {
        if buff.len() < (self.page_size as usize) {
            // todo convert to error in result
            panic!("buffer length is not equal to page_size");
        }

        self.file.seek(SeekFrom::Start(i * self.page_size))?;
        self.file.write_all(buff)?;

        return Ok(());
    }

    fn allocate_block(&mut self) {
    }
}