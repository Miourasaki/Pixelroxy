use std::{io};
use std::net::TcpStream;
use log::{error, info, Level, warn};
use uuid::Uuid;
use crate::libs::connect::connect::{ConnectState, PlayerConnect, PlayerInfo};
use crate::libs::connect::package::{BufferIndex};
use crate::libs::varint::{parse_varint};


mod status;


impl PlayerConnect {

    pub fn shake_hands(stream: TcpStream) -> io::Result<Self> {

        let mut connect = PlayerConnect {
            stream,
            proxy_stream: None,

            shake_hands_package: Vec::new(),
            player_info: None,
            zlib_enable: false,
            state: ConnectState::ShakeHands,
        };

        let player_id = Uuid::nil();
        let player_name = player_id.to_string();
        let client_ip = match connect.stream.peer_addr() {
            Ok(e) => e.to_string(),
            Err(_) => String::from("露米娅不知道哦<Unknown>")
        };
        let client_version;
        let target_host;
        let target_port;


        let package = connect.get_package()?;
        connect.shake_hands_package = package.original_package;


        let pkg = package.package;
        let mut package_index = BufferIndex(0);
        let package_id = pkg.get(package_index.get()).unwrap_or(&1);
        if *package_id != 0 {
            info!("客户端({}) 未发送握手数据包，被服务端拒绝连接<( ￣^￣)(θ(θ☆( >_<", client_ip);
            let _ = connect.stream.shutdown(std::net::Shutdown::Both);
            return Err(io::Error::from(io::ErrorKind::Other));
        } else {
            client_version = match parse_varint(&*pkg, &mut package_index){
                Some(i) => i  as u16,
                None => {
                    warn!("客户端({}) 发送了错误的数据包!<未知的游戏版本(#_<-)>", client_ip);
                    return Err(io::Error::from(io::ErrorKind::NotFound));
                }
            };
            let target_host_len = match pkg.get(package_index.get()) {
                Some(i) => i,
                None => {
                    warn!("客户端({}) 发送了错误的数据包!<未标记目标地址长度≧ ﹏ ≦>", client_ip);
                    return Err(io::Error::from(io::ErrorKind::NotFound));
                }
            };
            if *target_host_len >= 48 {
                warn!("客户端({}) 发送了错误的数据包!<目标地址过长___*( ￣皿￣)/#____>", client_ip);
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }

            let target_host_index = package_index.get();
            package_index.set(target_host_index + *target_host_len as usize);
            let target_host_bytes = &pkg[target_host_index..target_host_index + *target_host_len as usize];
            target_host = std::str::from_utf8(target_host_bytes).unwrap().to_string();
            target_port = u16::from_be_bytes([*pkg.get(package_index.get()).unwrap_or(&0), *pkg.get(package_index.get()).unwrap_or(&0)]);

            let player_info = PlayerInfo {
                player_name,
                player_id,
                client_ip,
                client_version,
                target_host: target_host.clone(),
                target_port,
            };
            connect.player_info = Some(player_info.clone());


            let game_state = match pkg.get(package_index.get()) {
                Some(i) => i,
                None => {
                    warn!("客户端({}) 发送了错误的数据包!<未知的握手状态|x🫱🏻🫲🏿x|>", player_info.client_ip);
                    return Err(io::Error::from(io::ErrorKind::NotFound));
                }
            };
            match *game_state {
                1 => { connect.state = ConnectState::Status;
                    info!("客户端({}) 与服务器握手成功(≧∇≦)ﾉ，连接状态 - ConnectState::Status，目标服务器[{}:{}]", player_info.client_ip, player_info.target_host, player_info.target_port);
                },
                2 => { connect.state = ConnectState::Proxy; },
                _ => {
                    warn!("客户端({}) 发送了错误的数据包!<未知的握手状态(〃＞目＜)>", player_info.client_ip);
                    return Err(io::Error::from(io::ErrorKind::Other));
                }
            }
        };

        Ok(connect)
    }


    pub fn listen(&mut self) {

        loop {
            let package = self.get_package();
            let result:bool;

            match package {
                Ok(package) => {
                    if package.package.is_empty() {
                        if self.state == ConnectState::Status {
                            info!("{} 断开连接", self.get_player_info());
                        }
                        if let Some(proxy_stream) = &mut self.proxy_stream {
                            let _ = proxy_stream.shutdown(std::net::Shutdown::Both);
                        }
                        break;
                    }

                    match self.state {
                        ConnectState::Status => { result = status::listen(package, self); }
                        _ => { result = false }
                    }

                }
                Err(e) => {
                    // 处理读取数据时发生的错误
                    if self.state == ConnectState::Status {
                        error!("{} 和服务器的Tcp连接发送错误: {}", self.get_player_info(), e);
                    }else {
                        info!("{} 和服务器的Tcp连接发送错误: {}", self.get_player_info(), e);
                    }
                    break;
                }
            }


            if !result { break; }
        }

        let _ = self.stream.shutdown(std::net::Shutdown::Both);
        // handle.thread().unpark();
    }


    pub fn close(&self, msg: &str, level: Level) {
        let _ = self.stream.shutdown(std::net::Shutdown::Both);
        match level {
            Level::Error => {
                error!("{} {}", self.get_player_info(), msg)
            }
            Level::Warn => {
                warn!("{} {}", self.get_player_info(), msg)
            }
            Level::Info => {
                info!("{} {}", self.get_player_info(), msg)
            }
            _ => ()
        }


    }


}