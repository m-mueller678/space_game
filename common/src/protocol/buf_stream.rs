use serde_json;
use std::io::{Read, Write};
use std::str::from_utf8;
use serde::{Deserialize, Serialize};

pub struct BufStream<S: Read + Write> {
    stream: S,
    buffer: [u8; 16_384],
    len: usize,
    read_to: usize
}

impl<S: Read + Write> BufStream<S> {
    pub fn new(stream: S) -> Self {
        BufStream {
            stream: stream,
            buffer: [0; 16_384],
            len: 0,
            read_to: 0,
        }
    }

    pub fn read<V: Deserialize>(&mut self) -> Option<Result<V, serde_json::Error>> {
        match self.stream.read(&mut self.buffer[self.len..]) {
            Ok(size) => {
                let new_len = self.len + size;
                if let Some(pos) = self.buffer[self.read_to..(new_len)].iter().position(|c| *c == b'\0') {
                    let abs_null_pos = pos + self.read_to;
                    let start_other = abs_null_pos + 1;
                    println!("read: {:?}", from_utf8(&self.buffer[..abs_null_pos]));
                    let res = serde_json::from_slice(&self.buffer[..abs_null_pos]);
                    for i in start_other..new_len {
                        self.buffer[i - start_other] = self.buffer[i];
                    }
                    self.len = new_len - start_other;
                    self.read_to = 0;
                    Some(res)
                } else {
                    self.read_to = self.len;
                    self.len = new_len;
                    None
                }
            },
            Err(e) => {
                Some(Err(e.into()))
            }
        }
    }

    pub fn write<V: Serialize>(&mut self, val: &V) -> Result<(), serde_json::Error> {
        serde_json::to_writer(&mut self.stream, val)?;
        self.stream.write(&[b'\0'])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BufStream;
    use std::io::{Cursor};
    use std::str::from_utf8;

    const DATA: [[u32; 4]; 3] = [[3, 43, 1, 33], [65, 2, 44, 1], [79, 54, 2, 5], ];
    const ENCODED: &'static str = "[3,43,1,33]\0[65,2,44,1]\0[79,54,2,5]\0";

    #[test]
    fn test_read() {
        let mut buf = ENCODED.to_string().into_bytes();
        let reader = Cursor::new(buf.as_mut_slice());
        let mut stream = BufStream::new(reader);
        for a in DATA.iter() {
            assert_eq!(*a, stream.read::<[u32; 4]>().unwrap().unwrap());
        }
    }

    #[test]
    fn test_write() {
        let mut buffer = [b' '; 1000];
        {
            let cursor = Cursor::new(&mut buffer[..]);
            let mut stream = BufStream::new(cursor);
            for a in DATA.iter() {
                stream.write(a).unwrap();
            }
        }
        let result = from_utf8(&buffer).unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>();
        assert_eq!(ENCODED, result);
    }
}
