use crate::types::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Row {
    attributes: BoltMap,
}

#[derive(Debug)]
pub struct Node {
    inner: BoltNode,
}

#[derive(Debug)]
pub struct Relation {
    inner: BoltRelation,
}

impl Row {
    pub fn new(fields: BoltList, data: BoltList) -> Self {
        let mut attributes = BoltMap::with_capacity(fields.len());
        for (field, value) in fields.into_iter().zip(data.into_iter()) {
            if let Ok(key) = field.try_into() {
                attributes.put(key, value);
            }
        }
        Row { attributes }
    }

    pub fn get<T: std::convert::TryFrom<BoltType>>(&self, key: &str) -> Option<T> {
        self.attributes.get(key)
    }
}

impl Node {
    pub fn new(inner: BoltNode) -> Self {
        Node { inner }
    }

    pub fn id(&self) -> i64 {
        self.inner.id.value
    }

    pub fn labels(&self) -> Vec<String> {
        self.inner.labels.iter().map(|l| l.to_string()).collect()
    }

    pub fn get<T: std::convert::TryFrom<BoltType>>(&self, key: &str) -> Option<T> {
        self.inner.get(key)
    }
}

impl Relation {
    pub fn new(inner: BoltRelation) -> Self {
        Relation { inner }
    }

    pub fn id(&self) -> i64 {
        self.inner.id.value
    }

    pub fn start_node_id(&self) -> i64 {
        self.inner.start_node_id.value
    }

    pub fn end_node_id(&self) -> i64 {
        self.inner.end_node_id.value
    }

    pub fn typ(&self) -> String {
        self.inner.typ.value.clone()
    }

    pub fn get<T: std::convert::TryFrom<BoltType>>(&self, key: &str) -> Option<T> {
        self.inner.get(key)
    }
}
