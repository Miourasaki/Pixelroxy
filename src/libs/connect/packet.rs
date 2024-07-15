use std::io::Write;
use crate::libs::connect::connect::PlayerConnect;
use crate::libs::varint::encode_varint;
use crate::libs::zlib::zlib_compress;

pub fn pack<'a>(bytes: &'a mut Vec<u8>, packet_id: u8, enable_zlib: bool) {

    bytes.insert(0, packet_id);

    if enable_zlib {
        if bytes.len() > 256 {
            let packet_len = encode_varint(bytes.len());
            *bytes = zlib_compress(&*bytes).unwrap();
            bytes.splice(..0, packet_len);
            bytes.splice(..0, encode_varint(bytes.len()));
        }else {
            bytes.insert(0, 0);
            bytes.splice(..0, encode_varint(bytes.len()));
        }
    }else {
        bytes.splice(..0, encode_varint(bytes.len()));
    }
}

impl PlayerConnect {
    // 将发包包装一下， 这样只需要把字节组和包id传进来，这样就可以自动封装，发包了
    pub fn send_pack<'a>(&'a mut self, bytes: &'a mut Vec<u8>, packet_id: u8) {

        pack(bytes, packet_id, self.zlib_enable);

        // debug!("服务器发送了数据包: {:?}",bytes);

        self.stream.write_all(bytes).unwrap();

    }
}