use std::cmp::Ordering;

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

trait EvaluateOperatorFunction {
    fn evaluate<I : Iterator<Item=PacketLiteralType>>(&self,iter : I) -> PacketLiteralType;
}

struct EvaluateOperator<T :  EvaluateOperatorFunction>(Vec<Packet>,T);
impl<T :  EvaluateOperatorFunction> PacketData for EvaluateOperator<T> {
    fn sum_child_versions(&self) -> u32 { self.0.iter().map(|packet| packet.sum_versions() ).sum::<u32>() }
    fn evaluate(&self) -> PacketLiteralType { self.1.evaluate( self.0.iter().map(|packet| packet.data.evaluate() ) ) }
}
impl<T :  EvaluateOperatorFunction + Default> EvaluateOperator<T> {
    fn new(children : Vec<Packet>) -> Box<Self> {
        Box::new( Self(children,Default::default()) )
    } 
}

#[derive(Default)]
struct SumFunction;
impl EvaluateOperatorFunction for SumFunction {
    fn evaluate<I : Iterator<Item=PacketLiteralType>>(&self,iter : I) -> PacketLiteralType { iter.sum() }
}

#[derive(Default)]
struct ProductFunction;
impl EvaluateOperatorFunction for ProductFunction {
    fn evaluate<I : Iterator<Item=PacketLiteralType>>(&self,mut iter : I) -> PacketLiteralType {
        let first = iter.next().expect("product operator requires at least one item");
        iter.fold(first,|prev,cur| prev*cur)
    }
}

#[derive(Default)]
struct MinimumFunction;
impl EvaluateOperatorFunction for MinimumFunction {
    fn evaluate<I : Iterator<Item=PacketLiteralType>>(&self,iter : I) -> PacketLiteralType { iter.min().expect("no children") }
}

#[derive(Default)]
struct MaximumFunction;
impl EvaluateOperatorFunction for MaximumFunction {
    fn evaluate<I : Iterator<Item=PacketLiteralType>>(&self,iter : I) -> PacketLiteralType { iter.max().expect("no children") }
}

struct ComparisonOperator(Packet,Packet,Ordering);
impl ComparisonOperator {
    fn new(mut children : Vec<Packet>, ordering : Ordering) -> Box<Self> {
        assert!(children.len() == 2, "incorrect number of children");
        let rhs = children.pop().unwrap();
        let lhs = children.pop().unwrap();
        Box::new( Self(lhs,rhs,ordering) )
    }
}

impl PacketData for ComparisonOperator {
    fn sum_child_versions(&self) -> u32 { self.0.sum_versions() + self.1.sum_versions() }
    fn evaluate(&self) -> PacketLiteralType {
        if self.0.evaluate().cmp(&self.1.evaluate()) == self.2 {
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
            0 => EvaluateOperator::<SumFunction>::new(packets),
            1 => EvaluateOperator::<ProductFunction>::new(packets),
            2 => EvaluateOperator::<MinimumFunction>::new(packets),
            3 => EvaluateOperator::<MaximumFunction>::new(packets),
            4 => panic!("somehow literal got to parse_operator"),
            5 => ComparisonOperator::new(packets,Ordering::Greater),
            6 => ComparisonOperator::new(packets,Ordering::Less),
            7 => ComparisonOperator::new(packets,Ordering::Equal),
            _ => unreachable!("invalid operator {}",id)
        } );
    }

    pub fn evaluate(&self) -> PacketLiteralType {
        self.data.evaluate()
    }

    pub fn sum_versions(&self) -> u32 {
        self.data.sum_child_versions() + (self.version as u32)
    }
}