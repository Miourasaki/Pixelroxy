use crate::libs::connect::package::BufferIndex;

pub fn parse_varint(data: &[u8], index: &mut BufferIndex) -> Option<u64> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;
    let mut byte_index = index.get();

    loop {
        if byte_index >= data.len() {
            return None; // 如果索引超出数组长度，返回 None
        }

        let byte = data[byte_index];
        result |= ((byte & 0x7F) as u64) << shift;
        shift += 7;
        byte_index += 1;

        if byte & 0x80 == 0 {
            index.set(byte_index); // 更新索引位置
            return Some(result);
        }

        if shift >= 64 {
            return None; // 防止溢出
        }
    }
}


pub fn encode_varint(mut value: usize) -> Vec<u8> {
    let mut result = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80; // Set the most significant bit to indicate there's more to come
        }
        result.push(byte);
        if value == 0 {
            break;
        }
    }
    result
}