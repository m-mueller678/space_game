use serde_json;
use std::io::{Read, Write, ErrorKind};
use std::io;
use std::str::from_utf8;
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct BufStream<S: Read + Write> {
    stream: S,
    buffer: [u8; 16_384],
    len: usize,
    read_to: usize
}

impl<S: fmt::Debug + Read + Write> fmt::Debug for BufStream<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        slice_to_string(&self.buffer[..self.len]).fmt(f)
    }
}

fn slice_to_string(data: &[u8]) -> String {
    from_utf8(data).map(|s| s.to_string()).unwrap_or_else(|_| { format!("{:?}", data) })
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
        if let Some(res) = self.try_deser() {
            Some(res)
        } else {
            if let Err(e) = self.read_to_buffer() {
                return Some(Err(e.into()))
            } else {
                self.try_deser()
            }
        }
    }

    pub fn write<V: Serialize>(&mut self, val: &V) -> Result<(), serde_json::Error> {
        serde_json::to_writer(&mut self.stream, val)?;
        self.stream.write_all(&[b'\0'])?;
        Ok(())
    }

    pub fn read_raw(&mut self) -> Option<Result<Vec<u8>, io::Error>> {
        if let Some(vec) = self.try_extract() {
            Some(Ok(vec))
        } else {
            if let Err(e) = self.read_to_buffer() {
                return Some(Err(e))
            } else {
                self.try_extract().map(|x| Ok(x))
            }
        }
    }

    pub fn write_raw(&mut self, msg: &[u8]) -> Result<(), io::Error> {
        self.stream.write_all(msg)?;
        self.stream.write_all(&[b'\0'])
    }

    fn try_deser<V: Deserialize>(&mut self) -> Option<Result<V, serde_json::Error>> {
        self.seek_null().and_then(|pos| {
            let res = serde_json::from_slice(&self.buffer[..pos]);
            self.shift_buffer(pos);
            Some(res)
        })
    }

    fn try_extract(&mut self) -> Option<Vec<u8>> {
        self.seek_null().and_then(|pos| {
            let vec = self.buffer[..pos].to_vec();
            self.shift_buffer(pos);
            Some(vec)
        })
    }

    fn read_to_buffer(&mut self) -> Result<(), io::Error> {
        match self.stream.read(&mut self.buffer[self.len..]) {
            Ok(size) => {
                self.len += size;
                debug!("read {} bytes, buffer: {:?}", size, slice_to_string(&self.buffer[..self.len]));
                if size == 0 {
                    if self.len == self.buffer.len() {
                        Err(io::Error::new(ErrorKind::Other, "buffer full"))
                    } else {
                        Err(io::Error::new(ErrorKind::BrokenPipe, "0 bytes read"))
                    }
                } else {
                    Ok(())
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    Ok(())
                } else {
                    warn!("read error: {}", e);
                    Err(e)
                }
            }
        }
    }

    fn shift_buffer(&mut self, null_pos: usize) {
        let next_start = null_pos + 1;
        for i in (next_start)..self.len {
            self.buffer[i - next_start] = self.buffer[i];
        }
        self.len -= next_start;
        self.read_to = 0;
    }

    fn seek_null(&mut self) -> Option<usize> {
        let r = self.buffer[self.read_to..self.len].iter().position(|x| *x == b'\0').map(|p| p + self.read_to);
        self.read_to = r.unwrap_or(self.len);
        r
    }

    pub fn raw(&self) -> &S {
        &self.stream
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
