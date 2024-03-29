use std::io::{Cursor, self};

use tokio::io::{BufWriter, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use mini_redis::{Frame, Result};
use bytes::{Buf, BytesMut};
use mini_redis::frame::Error::Incomplete;

struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
    cursor: usize,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection { 
            stream: BufWriter::new(socket), 
            buffer: BytesMut::with_capacity(4096),
            cursor: 0
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.prase_frame()? {
                return Ok(Some(frame));
            }
            
            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor * 2, 0);
            }

            let n = self.stream.read(
                &mut self.buffer[self.cursor..]
            ).await?;

            if 0 == n {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                self.cursor += n;
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();
                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!(),
        }

        self.stream.flush().await;

        Ok(())
    }

    fn prase_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            },
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn write_decimal(&mut self, val: u64) -> Result<()> {
        use std::io::Write;

        let mut buf = [0u8; 12];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val);

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}