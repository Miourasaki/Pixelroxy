use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use log::{debug, info};
use serde_json::json;
use crate::libs::connect::package::{BufferIndex, ConnectPackage, get_package};
use crate::libs::connect::packet::pack;
use crate::server::auth::PlayerConnect;
use crate::libs::varint::encode_varint;


impl PlayerConnect {

    fn send_motd(&mut self) {

        info!("{} 请求PingList数据☆⌒(*＾-゜)v THX!!，目标服务器[{}:{}]", self.get_player_info(), self.player_info.clone().unwrap().target_host, self.player_info.clone().unwrap().target_port);


        self.proxy_stream = Some(TcpStream::connect("play.hypixel.net:25565").unwrap());

        /*
        向 play.hypixel.net:25565 请求发送示例
        0000   14 cf 92 36 6e 9e 84 5c f3 98 ad 91 08 00 45 00   ...6n..\......E.
        0010   00 40 e6 a7 40 00 40 06 00 00 c0 a8 b1 0c d1 de   .@..@.@.........
        0020   73 1b 20 61 63 dd 63 e9 14 38 39 64 ea 36 50 18   s. ac.c..89d.6P.
        0030   02 03 b6 e1 00 00 17 00 ff 05 10 70 6c 61 79 2e   ...........play.
        0040   68 79 70 69 78 65 6c 2e 6e 65 74 63 dd 01         hypixel.netc..
        */

        if let Some(target_stream) = &mut self.proxy_stream {
            let mut shake_hand_package = vec![0x10, 0x70, 0x6c, 0x61, 0x79, 0x2e, 0x68, 0x79, 0x70, 0x69, 0x78, 0x65, 0x6c, 0x2e, 0x6e, 0x65, 0x74, 0x63, 0xdd, 0x01 ];
            shake_hand_package.splice(..0, encode_varint(self.player_info.clone().unwrap().client_version as usize));
            pack(&mut shake_hand_package, 0, false);
            target_stream.write_all(&shake_hand_package).unwrap();
            target_stream.write_all(&[1u8, 0u8]).unwrap();



            thread::sleep(Duration::from_secs(1));
            if let Ok(package) = get_package(&target_stream, false) {
                println!("{:?}", package);
                self.stream.write_all(&package.original_package).unwrap();

            }
        }

    }
}

pub fn listen(pak: ConnectPackage,connect: &mut PlayerConnect) -> bool {

    let package = pak.package;

    let mut package_index = BufferIndex(0);
    let package_id = *package.get(package_index.get()).unwrap_or(&2);
    debug!("<STATUS>接收到{:#x}数据包{:?}，长度为{}", package_id, package, package.len());

    if package_id == 0 { connect.send_motd() };
    if package_id == 1 {
        let mut packet = package;

        if let Some(target_stream) = &mut connect.proxy_stream {
            target_stream.write_all(&pak.original_package).unwrap();
            if let Ok(package) = get_package(&target_stream, false) {
                connect.stream.write_all(&package.original_package).unwrap()
            }

        }else {
            packet.splice(..0, encode_varint(packet.len()));
            connect.stream.write_all(&packet).unwrap();
        }


    }

    true
}