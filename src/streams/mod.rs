use serde::{
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

pub trait StreamDatum {
    const ID: u16;
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Chunk2<TA, TB> {
    A(Vec<TA>),
    B(Vec<TB>),
}

impl<TA, TB> Chunk2<TA, TB> {
    pub fn is_a(&self) -> bool {
        use Chunk2::{A, B};
        match *self {
            A(_) => true,
            B(_) => false,
        }
    }
    pub fn is_b(&self) -> bool {
        use Chunk2::{A, B};
        match *self {
            A(_) => false,
            B(_) => true,
        }
    }
    pub fn a(&self) -> Option<&Vec<TA>> {
        use Chunk2::{A, B};
        match self {
            A(aa) => Some(aa),
            B(_) => None,
        }
    }
    pub fn b(&self) -> Option<&Vec<TB>> {
        use Chunk2::{A, B};
        match self {
            A(_) => None,
            B(bb) => Some(bb),
        }
    }
    pub fn chunk<AF, BF, T>(self, f: AF, g: BF) -> Vec<T>
    where
        AF: FnMut(TA) -> T,
        BF: FnMut(TB) -> T,
    {
        use Chunk2::{A, B};
        match self {
            A(aa) => aa.into_iter().map(f).collect::<Vec<T>>(),
            B(bb) => bb.into_iter().map(g).collect::<Vec<T>>(),
        }
    }
    pub fn len(&self) -> usize {
        use Chunk2::{A, B};
        match self {
            A(aa) => aa.len(),
            B(bb) => bb.len(),
        }
    }
}

impl<TA, TB> Serialize for Chunk2<TA, TB>
where
    TA: StreamDatum + Serialize,
    TB: StreamDatum + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use Chunk2::{A, B};
        match self {
            A(aa) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&TA::ID)?;
                seq.serialize_element(aa)?;
                seq.end()
            }
            B(bb) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&TB::ID)?;
                seq.serialize_element(bb)?;
                seq.end()
            }
        }
    }
}

struct U16Visitor;

impl<'de> Visitor<'de> for U16Visitor {
    type Value = u16;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting an unsigned 16 bit integer")
    }
}

struct Chunk2Visitor<TA, TB> {
    a_: std::marker::PhantomData<TA>,
    b_: std::marker::PhantomData<TB>,
}

impl<'de, TA, TB> Visitor<'de> for Chunk2Visitor<TA, TB>
where
    TA: for<'a> Deserialize<'a> + StreamDatum + fmt::Debug,
    TB: for<'a> Deserialize<'a> + StreamDatum + fmt::Debug,
{
    type Value = Chunk2<TA, TB>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum Chunk<A, B>")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Chunk2<TA, TB>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let type_id: u16 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        if type_id == TA::ID {
            println!("found {}", type_id);
            let vals: Vec<TA> = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            println!("result len={}", vals.len());
            Ok(Chunk2::A(vals))
        } else if type_id == TB::ID {
            println!("found {}", type_id);
            let vals: Vec<TB> = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            println!("result len={}", vals.len());
            Ok(Chunk2::B(vals))
        } else {
            Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &self))
        }
    }
}

impl<'de, TA, TB> Deserialize<'de> for Chunk2<TA, TB>
where
    TA: StreamDatum + for<'a> Deserialize<'a> + fmt::Debug,
    TB: StreamDatum + for<'a> Deserialize<'a> + fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vis = Chunk2Visitor {
            a_: std::marker::PhantomData,
            b_: std::marker::PhantomData,
        };
        deserializer.deserialize_seq(vis)
    }
}
