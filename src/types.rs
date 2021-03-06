pub mod boolean;
pub mod integer;
pub mod list;
pub mod map;
pub mod node;
pub mod null;
pub mod relation;
pub mod string;
pub use boolean::BoltBoolean;
pub use integer::BoltInteger;
pub use list::BoltList;
pub use map::BoltMap;
pub use node::BoltNode;
pub use null::BoltNull;
pub use relation::BoltRelation;
pub use string::BoltString;

use crate::errors::*;
use bytes::Bytes;
use core::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BoltType {
    String(BoltString),
    Boolean(BoltBoolean),
    Map(BoltMap),
    Null(BoltNull),
    Integer(BoltInteger),
    List(BoltList),
    Node(BoltNode),
    Relation(BoltRelation),
}

impl Display for BoltType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BoltType::String(s) => s.to_string(),
            _ => "to_string not implemented".to_owned(),
        };
        write!(f, "{}", value)
    }
}

impl Hash for BoltType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            BoltType::String(t) => t.hash(state),
            BoltType::Boolean(t) => t.hash(state),
            BoltType::Null(t) => t.hash(state),
            BoltType::Integer(t) => t.hash(state),
            BoltType::List(t) => t.hash(state),
            BoltType::Node(_) => panic!("node not hashed"),
            BoltType::Map(_) => panic!("map not hashed"),
            BoltType::Relation(_) => panic!("relation not hashed"),
        }
    }
}

impl TryInto<Bytes> for BoltType {
    type Error = Error;
    fn try_into(self) -> Result<Bytes> {
        match self {
            BoltType::Null(t) => t.try_into(),
            BoltType::Boolean(t) => t.try_into(),
            BoltType::Integer(t) => t.try_into(),
            BoltType::String(t) => t.try_into(),
            BoltType::List(t) => t.try_into(),
            BoltType::Map(t) => t.try_into(),
            BoltType::Node(t) => t.try_into(),
            BoltType::Relation(t) => t.try_into(),
        }
    }
}

impl TryFrom<Rc<RefCell<Bytes>>> for BoltType {
    type Error = Error;
    fn try_from(input: Rc<RefCell<Bytes>>) -> Result<BoltType> {
        let bolt_type = match input {
            input if BoltInteger::can_parse(input.clone()) => BoltType::Integer(input.try_into()?),
            input if BoltString::can_parse(input.clone()) => BoltType::String(input.try_into()?),
            input if BoltList::can_parse(input.clone()) => BoltType::List(input.try_into()?),
            input if BoltMap::can_parse(input.clone()) => BoltType::Map(input.try_into()?),
            input if BoltNode::can_parse(input.clone()) => BoltType::Node(input.try_into()?),
            input if BoltBoolean::can_parse(input.clone()) => BoltType::Boolean(input.try_into()?),
            input if BoltRelation::can_parse(input.clone()) => {
                BoltType::Relation(input.try_into()?)
            }
            _ => return Err(Error::UnknownType(format!("{:#04X?}", input.borrow()))),
        };
        Ok(bolt_type)
    }
}
