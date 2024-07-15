use std::error::Error;
use std::net::{TcpListener};
use std::thread;
use log::{info};
use crate::config::Configure;
use crate::libs::connect::connect::PlayerConnect;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn from(configure: Configure) -> Result<Self, Box<dyn Error>> {
        let server_address = format!("{}:{}", configure.host, configure.port);

        let server = TcpListener::bind(server_address.as_str())?;

        info!("服务器初始化完成，监听地址 —— {}", server_address);

        Ok(Self {
            listener: server,
        })
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                thread::spawn(move || {

                    if let Ok(mut connect) = PlayerConnect::shake_hands(stream) {
                        connect.listen();
                    }

                });
            }
        }
    }
}




