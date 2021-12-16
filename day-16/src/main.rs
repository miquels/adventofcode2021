use std::io::{self, Read};

struct Bitstream {
    nibble: u8,
    npos:   u8,
    buffer: String,
    eof:    bool,
}

impl Bitstream {
    fn new() -> Bitstream {
        Bitstream{ nibble: 0, npos: 0, buffer: String::new(), eof: false }
    }

    fn eof(&mut self) -> io::Result<bool> {
        self.fill_buffer()?;
        Ok(self.eof && self.buffer.len() < 2)
    }

    fn fill_buffer(&mut self) -> io::Result<()> {
        if !self.eof {
            let mut buf = [0u8; 1];
            while self.buffer.len() < 4 {
                if io::stdin().read(&mut buf)? == 0 {
                    self.eof = true;
                    break;
                }
                if buf[0] == b'\n' {
                    continue;
                }
                self.buffer.push(buf[0] as char);
            }
        }
        Ok(())
    }

    fn read_bit(&mut self) -> io::Result<Option<bool>> {
        if self.npos == 0 {
            self.fill_buffer()?;
            if self.buffer.len() == 0 {
                return Ok(None);
            }
            let ns = &self.buffer[..1];
            self.nibble = u8::from_str_radix(ns, 16)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}: {}", ns, e)))?;
            self.npos = 1 << 3;
            self.buffer.remove(0);
        }
        let b = (self.nibble & self.npos) > 0;
        self.npos >>= 1;
        Ok(Some(b))
    }

    fn read_bits(&mut self, num_bits: u8) -> io::Result<Option<u32>> {
        let mut n = 0;
        for x in 0 .. num_bits {
            let b = match self.read_bit()? {
                Some(b) => b,
                None => {
                    if x == 0 {
                        return Ok(None);
                    }
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
            };
            n = (n << 1) | (b as u32);
        }
        Ok(Some(n))
    }
}

struct PacketReader<'a> {
    nbits:      Option<u32>,
    stream:     &'a mut Bitstream,
    bits_read:  u32,
    eof_ok:      bool,
}

impl<'a> PacketReader<'a> {
    fn new<'s>(stream: &'s mut Bitstream) -> PacketReader<'s> {
        PacketReader {
            nbits:      None,
            stream,
            bits_read:  0,
            eof_ok:      true,
        }
    }

    fn limit_bits<'b>(&'b mut self, limit: u32) -> PacketReader<'b> {
        PacketReader {
            nbits:      Some(limit),
            stream:     self.stream,
            bits_read:  0,
            eof_ok:     true,
        }
    }

    fn read_bits(&mut self, num_bits: u8) -> io::Result<u32> {
        match self.nbits {
            Some(0) => return Err(io::ErrorKind::UnexpectedEof.into()),
            Some(ref mut n) => {
                if *n < num_bits as u32 {
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
                *n -= num_bits as u32;
            },
            None => {},
        }
        let n = match self.stream.read_bits(num_bits)? {
            Some(n) => {
                self.bits_read += num_bits as u32;
                n
            },
            None => return Err(io::ErrorKind::UnexpectedEof.into()),
        };
        Ok(n)
    }

    fn eof(&mut self) -> io::Result<bool> {
        if self.nbits == Some(0) {
            return Ok(true);
        }
        self.stream.eof()
    }

    fn read_packet(&mut self) -> io::Result<Option<Packet>> {
        if self.eof_ok && self.eof()? {
            return Ok(None);
        }
        let version = self.read_bits(3)? as u8;
        let type_id = self.read_bits(3)? as u8;
        match type_id {
            Packet::LITERAL => {
                let literal = self.read_literal()?;
                Ok(Some(Packet {
                    version,
                    type_id,
                    data: PacketData::Literal(literal),
                }))
            },
            _ => {
                let packets = self.read_subpackets()?;
                Ok(Some(Packet {
                    version,
                    type_id,
                    data: PacketData::Packets(packets),
                }))
            }
        }
    }

    fn read_literal(&mut self) -> io::Result<u64> {
        let mut val = 0;
        while {
            let last = self.read_bits(1)? == 0;
            val = (val << 4) | (self.read_bits(4)? as u64);
            !last
        } {}
        Ok(val)
    }

    fn read_packets(&mut self, amount: Option<u32>) -> io::Result<Vec<Packet>> {
        let mut pv = Vec::new();
        if let Some(amount) = amount {
            for _ in 0 .. amount {
                match self.read_packet()? {
                    Some(p) => pv.push(p),
                    None => return Err(io::ErrorKind::UnexpectedEof.into()),
                }
            }
        } else {
            while let Some(p) = self.read_packet()? {
                pv.push(p);
            }
        }
        Ok(pv)
    }

    fn read_subpackets(&mut self) -> io::Result<Vec<Packet>> {
        match self.read_bits(1)? != 0 {
            false => {
                let limit = self.read_bits(15)?;
                let mut reader = self.limit_bits(limit);
                let packets = reader.read_packets(None)?;
                let bits_read = reader.bits_read;
                if bits_read != limit {
                    println!("read_subpacket: needed {} bits, got {} bits", limit, bits_read);
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
                self.bits_read += bits_read;
                if let Some(ref mut nbits) = self.nbits {
                    *nbits -= bits_read;
                }
                Ok(packets)
            },
            true => {
                let limit = self.read_bits(11)?;
                self.read_packets(Some(limit))
            }
        }
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Packets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version:   u8,
    type_id:   u8,
    data:      PacketData,
}

impl Packet {
    const SUM: u8 = 0u8;
    const PRODUCT: u8 = 1u8;
    const MINIMUM: u8 = 2u8;
    const MAXIMUM: u8 = 3u8;
    const LITERAL: u8 = 4u8;
    const GREATER: u8 = 5u8;
    const LESS: u8 = 6u8;
    const EQUAL: u8 = 7u8;

    fn version_sum(&self) -> u32 {
        let mut sum = 0;
        sum += self.version as u32;
        match self.data {
            PacketData::Literal(_) => {},
            PacketData::Packets(ref packets) => {
                for packet in packets {
                    sum += packet.version_sum();
                }
            }
        }
        sum
    }

    fn value(&self) -> u64 {
        match &self.data {
            PacketData::Literal(lit) => *lit,
            PacketData::Packets(packets) => {
                match self.type_id {
                    Packet::SUM => packets.iter().map(|p| p.value()).sum(),
                    Packet::PRODUCT => packets.iter().map(|p| p.value()).product::<u64>(),
                    Packet::MINIMUM => packets.iter().map(|p| p.value()).min().unwrap_or(0),
                    Packet::MAXIMUM => packets.iter().map(|p| p.value()).max().unwrap_or(0),
                    Packet::GREATER => (packets[0].value() > packets[1].value()) as u64,
                    Packet::LESS => (packets[0].value() < packets[1].value()) as u64,
                    Packet::EQUAL => (packets[0].value() == packets[1].value()) as u64,
                    other => {
                        println!("unknown packet type {},skipping", other);
                        0
                    },
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut stream = Bitstream::new();
    let mut reader = PacketReader::new(&mut stream);
    let packet = reader.read_packet()?.unwrap();
    println!("version_sum: {}", packet.version_sum());
    println!("value: {}", packet.value());

    Ok(())
}
