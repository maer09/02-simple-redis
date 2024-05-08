mod decode;
mod encode;

use enum_dispatch::enum_dispatch;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> RespFrame;
}

#[enum_dispatch(RespEncode)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    // BigNumber,
    Map(RespMap),
    Set(RespSet),
}

pub struct SimpleString(String);
pub struct SimpleError(String);
pub struct BulkString(Vec<u8>);
pub struct RespNull;
pub struct RespArray(Vec<RespFrame>);
pub struct RespNullArray;
pub struct RespNullBulkString;
pub struct RespMap(HashMap<String, RespFrame>);
pub struct RespSet(HashSet<RespFrame>);

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespMap {
    type Target = HashMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespSet {
    type Target = HashSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

// impl From<SimpleString> for RespFrame {
//     fn from(value: SimpleString) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<SimpleError> for RespFrame {
//     fn from(value: SimpleError) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<i64> for RespFrame {
//     fn from(value: i64) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<BulkString> for RespFrame {
//     fn from(value: BulkString) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<RespNullBulkString> for RespFrame {
//     fn from(value: RespNullBulkString) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<RespArray> for RespFrame {
//     fn from(value: RespArray) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<RespNull> for RespFrame {
//     fn from(value: RespNull) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<RespNullArray> for RespFrame {
//     fn from(value: RespNullArray) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<bool> for RespFrame {
//     fn from(value: bool) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<RespMap> for RespFrame {
//     fn from(value: RespMap) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }

// impl From<RespSet> for RespFrame {
//     fn from(value: RespSet) -> Self {
//         RespFrame::SimpleString(value)
//     }
// }
