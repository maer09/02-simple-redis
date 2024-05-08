use crate::{RespDecode, SimpleString};

// +OK\r\n
impl RespDecode for SimpleString {
    fn decode(_buf: Self) -> crate::RespFrame {
        todo!()
    }
}
