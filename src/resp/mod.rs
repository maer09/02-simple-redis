use std::collections::{HashMap, HashSet};

pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: &[u8]) -> RespFrame;
}

pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(Vec<u8>),
    NullBulkString(RespNullBulkString),
    Array(Vec<RespFrame>),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    // BigNumber,
    Map(HashMap<String, RespFrame>),
    Set(HashSet<RespFrame>),
}

#[allow(dead_code)]
pub struct SimpleString(String);
#[allow(dead_code)]
pub struct SimpleError(String);
pub struct RespNull;
pub struct RespNullArray;
pub struct RespNullBulkString;
