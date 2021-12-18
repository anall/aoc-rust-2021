pub struct BitIterator<T: Iterator<Item=u8>> {
    iter : T,
    cur : Option<(u8,usize)>,
    ct : usize,
}

impl<T: Iterator<Item=u8>> BitIterator<T> {
    pub fn new(iter : T) -> Self {
        Self { iter, cur : None, ct : 0 }
    }

    pub fn next_integer(&mut self, ct : usize) -> Option<u32> {
        let mut result : u32 = 0;
        for _ in 0 .. ct {
            result = result << 1 | (self.next()? as u32);
        }
        return Some(result);
    }

    pub fn bits_read(&self) -> usize {
        self.ct
    }
}

impl<T: Iterator<Item=u8>> Iterator for BitIterator<T> {
    type Item = u8; // makes things easier

    fn next(&mut self) -> Option<u8> {
        if let Some((cur_val,mut cur_idx)) = self.cur.take() {
            let result = (cur_val >> cur_idx) & 1;
            if cur_idx != 0 {
                cur_idx -= 1;
                self.cur = Some( (cur_val,cur_idx) );
            }
            self.ct += 1;
            return Some(result);
        } else if let Some(cur_val) = self.iter.next() {
            let mut cur_idx = 7;
            let result = (cur_val >> cur_idx) & 1;
            cur_idx -= 1;
            self.cur = Some( (cur_val,cur_idx) );
            self.ct += 1;
            return Some(result);
        } else {
            return None;
        }
    }
}

type PacketLiteralType = i64;

trait PacketData {
    fn sum_child_versions(&self) -> u32;
    fn evaluate(&self) -> PacketLiteralType;
}

struct LiteralPacket(PacketLiteralType);
impl PacketData for LiteralPacket {
    fn sum_child_versions(&self) -> u32 { 0 }
    fn evaluate(&self) -> PacketLiteralType { self.0 }
}

// yes, this duplicates sum_child_versions, sigh
struct SumOperator(Vec<Packet>);
impl PacketData for SumOperator {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType { self.0.iter().map(|packet| packet.data.evaluate() ).sum::<PacketLiteralType>() }
}

struct ProductOperator(Vec<Packet>);
impl PacketData for ProductOperator {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType {
        let mut iter = self.0.iter().map(|packet| packet.data.evaluate() );
        let first = iter.next().expect("product operator requires at least one item");
        iter.fold(first,|prev,cur| prev*cur)
    }
}

struct MinimumOperator(Vec<Packet>);
impl PacketData for MinimumOperator {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType {
        self.0.iter().map(|packet| packet.data.evaluate() ).min().expect("got 0 children")
    }
}

struct MaximumOperatror(Vec<Packet>);
impl PacketData for MaximumOperatror {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType {
        self.0.iter().map(|packet| packet.data.evaluate() ).max().expect("got 0 children")
    }
}

struct GreaterThanOperator(Vec<Packet>);
impl PacketData for GreaterThanOperator {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType {
        assert!(self.0.len() == 2,"invalid number of children");
        if self.0[0].evaluate() > self.0[1].evaluate() {
            1
        } else {
            0
        }
    }
}

struct LessThanOperator(Vec<Packet>);
impl PacketData for LessThanOperator {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType {
        assert!(self.0.len() == 2,"invalid number of children");
        if self.0[0].evaluate() < self.0[1].evaluate() {
            1
        } else {
            0
        }
    }
}

struct EqualOperator(Vec<Packet>);
impl PacketData for EqualOperator {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType {
        assert!(self.0.len() == 2,"invalid number of children");
        if self.0[0].evaluate() == self.0[1].evaluate() {
            1
        } else {
            0
        }
    }
}

impl std::fmt::Debug for dyn PacketData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<PacketData>")
    }
}

#[derive(Debug)]
pub struct Packet {
    version : u8,
    data : Box<dyn PacketData>
}
impl Packet {
    pub fn parse<T: Iterator<Item=u8>>(iter : &mut BitIterator<T>) -> Option<Self> {
        let version = iter.next_integer(3)? as u8;
        let packet_type = iter.next_integer(3)? as u8;
        let data = match packet_type {
            4 => Self::parse_literal(iter)?,
            _ => Self::parse_operator(packet_type,iter)?,
        };
        Some( Self { version, data } )
    }

    fn parse_literal<T: Iterator<Item=u8>>(iter : &mut BitIterator<T>) -> Option<Box<dyn PacketData>> {
        let mut next_group = true;
        let mut result : PacketLiteralType = 0;
        while next_group {
            next_group = iter.next()? == 1;
            result = result << 4 | ( iter.next_integer(4)? as PacketLiteralType );
        }
        return Some( Box::new( LiteralPacket(result) ) );
    }

    fn parse_operator<T: Iterator<Item=u8>>(id : u8, iter : &mut BitIterator<T>) -> Option<Box<dyn PacketData>> {
        let length_type = iter.next()?;
        let mut packets : Vec<Packet> = Vec::new();
        if length_type == 0 {
            let bit_length = iter.next_integer(15)? as usize;
            let bit_start = iter.bits_read();
            while iter.bits_read()-bit_start < bit_length {
                packets.push( Self::parse(iter)? );
            }
            assert_eq!(iter.bits_read()-bit_start, bit_length);
        } else {
            for _ in 0 .. iter.next_integer(11)? {
                packets.push( Self::parse(iter)? );
            }
        }
        return Some( match id {
            0 => Box::new( SumOperator(packets) ),
            1 => Box::new( ProductOperator(packets) ),
            2 => Box::new( MinimumOperator(packets) ),
            3 => Box::new( MaximumOperatror(packets) ),
            // literal
            5 => Box::new( GreaterThanOperator(packets) ),
            6 => Box::new( LessThanOperator(packets) ),
            7 => Box::new( EqualOperator(packets) ),
            _ => unimplemented!("invalid operator {}",id)
            //_ => Box::new( OperatorPacket(id,packets) ),
        } );
    }

    pub fn evaluate(&self) -> PacketLiteralType {
        self.data.evaluate()
    }

    pub fn sum_versions(&self) -> u32 {
        let recursed_version = self.data.sum_child_versions();
        recursed_version + (self.version as u32)
    }
}