use std::io;
use std::io::Read;
use std::net::TcpStream;
use log::debug;
use crate::libs::connect::connect::PlayerConnect;
use crate::libs::varint::{encode_varint, parse_varint};
use crate::libs::zlib::zlib_decompress;


pub struct BufferIndex(pub usize);

impl BufferIndex {
    pub fn get(&mut self) -> usize {
        let result = self.0;
        self.0 += 1;
        result
    }

    pub fn get_len(&mut self, len: usize) -> usize {
        let result = self.0;
        self.0 += len;
        result
    }
    pub fn just_get(&mut self) -> usize {
        self.0
    }


    pub fn set(&mut self, target: usize) {
        self.0 = target
    }
}

#[derive(Debug)]
pub struct ConnectPackage {
    pub package_len: usize,
    pub package: Vec<u8>,
    pub original_package: Vec<u8>
}


impl PlayerConnect {
    pub fn get_package(&self) -> io::Result<ConnectPackage> {
        get_package(&self.stream, self.zlib_enable)
    }
}

pub fn get_package(mut stream: &TcpStream, zlib_enable:bool) -> io::Result<ConnectPackage> {


    let mut package_len = parse_varint_from_stream(stream)?;
    while package_len == 0 {
        package_len = parse_varint_from_stream(stream)?;
    }

    let mut connect_package = ConnectPackage {
        package_len,
        package: vec![],
        original_package: vec![]
    };

    if package_len == usize::MAX {
        return Ok(connect_package)

    }
    let mut package = vec![0u8; package_len]; // 创建一个缓冲区来存储从客户端接收的数据

    stream.read(&mut package)?;
    debug!("正在解析数据包 长度：{} 主体：{:?}", package_len, package);

    let mut varint_package_len = encode_varint(package_len);
    varint_package_len.extend(package.clone());
    connect_package.original_package = varint_package_len;

    if zlib_enable {

        let mut package_index = BufferIndex(0);
        let unzlib_package_len = parse_varint(&*package, &mut package_index);
        if let Some(len) = unzlib_package_len {
            if len == 0 {
                connect_package.package = package[package_index.get()..package.len()].to_owned();
            }
            let zlib_package = &package[package_index.get()..package.len()];
            let unzlib_package = zlib_decompress(zlib_package).unwrap();
            if unzlib_package.len() == len as usize {
                connect_package.package = unzlib_package;
            }
        }else {
            connect_package.package = vec![];
            connect_package.original_package = vec![];
        }
    }else {
        connect_package.package = Vec::from(&package[..package.len()]);
    }

    return Ok(connect_package)


}


pub fn parse_varint_from_stream(mut stream: &TcpStream) -> io::Result<usize> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;

    loop {
        let mut byte_vec = vec![0u8; 1];
        let buffer_len = stream.read(&mut byte_vec)?;
        if buffer_len == 0 {
            return Ok(usize::MAX)
        }
        let byte = byte_vec[0];

        result |= ((byte & 0x7F) as u64) << shift;
        shift += 7;

        if byte & 0x80 == 0 {
            return Ok(result as usize);
        }

        if shift >= 64 {
            return Err(io::Error::from(io::ErrorKind::Other));
        }
    }
}
