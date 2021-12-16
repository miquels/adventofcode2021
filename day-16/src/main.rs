use std::io::{self, Read};

struct Bitstream {
    nibble: u8,
    npos:   u8,
    bits_read: u32,
}

impl Bitstream {
    fn new() -> Bitstream {
        Bitstream{ nibble: 0, npos: 0, bits_read: 0 }
    }

    fn read_bit(&mut self) -> io::Result<bool> {
        if self.npos == 0 {
            let mut buf = [0u8; 1];
            loop {
                if io::stdin().read(&mut buf)? == 0 {
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
                if buf[0] != b'\n' {
                    break;
                }
            }
            let ns = String::from(buf[0] as char);
            self.nibble = u8::from_str_radix(&ns, 16)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}: {}", ns, e)))?;
            self.npos = 1 << 3;
        }
        let b = (self.nibble & self.npos) > 0;
        self.npos >>= 1;
        self.bits_read += 1;
        Ok(b)
    }

    fn read_bits(&mut self, num_bits: u8) -> io::Result<u32> {
        let mut n = 0;
        for _ in 0 .. num_bits {
            let b = self.read_bit()?;
            n = (n << 1) | (b as u32);
        }
        Ok(n)
    }
}

struct PacketReader<'a> {
    stream:     &'a mut Bitstream,
}

impl<'a> PacketReader<'a> {
    fn new<'s>(stream: &'s mut Bitstream) -> PacketReader<'s> {
        PacketReader { stream }
    }

    fn read_packet(&mut self) -> io::Result<Packet> {
        let version = self.stream.read_bits(3)? as u8;
        let type_id = self.stream.read_bits(3)? as u8;
        match type_id {
            Packet::LITERAL => {
                let literal = self.read_literal()?;
                Ok(Packet {
                    version,
                    type_id,
                    literal,
                    subpackets: Vec::new(),
                })
            },
            _ => {
                let subpackets = self.read_subpackets()?;
                Ok(Packet {
                    version,
                    type_id,
                    literal: 0,
                    subpackets,
                })
            }
        }
    }

    fn read_literal(&mut self) -> io::Result<u64> {
        let mut val = 0;
        while {
            let last = self.stream.read_bits(1)? == 0;
            val = (val << 4) | (self.stream.read_bits(4)? as u64);
            !last
        } {}
        Ok(val)
    }

    fn read_subpackets(&mut self) -> io::Result<Vec<Packet>> {
        let mut packets = Vec::new();
        match self.stream.read_bits(1)? != 0 {
            false => {
                let nbits = self.stream.read_bits(15)?;
                let goal = self.stream.bits_read + nbits;
                while self.stream.bits_read != goal {
                    packets.push(self.read_packet()?);
                }
                Ok(packets)
            },
            true => {
                let npackets = self.stream.read_bits(11)?;
                for _ in 0 .. npackets {
                    packets.push(self.read_packet()?);
                }
                Ok(packets)
            }
        }
    }
}

#[derive(Debug)]
struct Packet {
    version:    u8,
    type_id:    u8,
    literal:    u64,
    subpackets: Vec<Packet>,
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
        for packet in &self.subpackets {
            sum += packet.version_sum();
        }
        sum
    }

    fn value(&self) -> u64 {
        let packets = &self.subpackets;
        match self.type_id {
            Packet::SUM => packets.iter().map(|p| p.value()).sum(),
            Packet::PRODUCT => packets.iter().map(|p| p.value()).product::<u64>(),
            Packet::MINIMUM => packets.iter().map(|p| p.value()).min().unwrap_or(0),
            Packet::MAXIMUM => packets.iter().map(|p| p.value()).max().unwrap_or(0),
            Packet::LITERAL => self.literal,
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

fn main() -> io::Result<()> {
    let mut stream = Bitstream::new();
    let mut reader = PacketReader::new(&mut stream);
    let packet = reader.read_packet()?;
    println!("version_sum: {}", packet.version_sum());
    println!("value: {}", packet.value());

    Ok(())
}
