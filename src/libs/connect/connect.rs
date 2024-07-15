use std::net::TcpStream;
use uuid::Uuid;

#[derive(PartialEq)]
pub enum ConnectState {
    Status,
    Proxy,
    ShakeHands
}

pub struct PlayerConnect {
    pub stream: TcpStream,
    pub proxy_stream: Option<TcpStream>,

    pub shake_hands_package: Vec<u8>,

    pub player_info: Option<PlayerInfo>,

    pub zlib_enable: bool,
    pub state: ConnectState
}
#[derive(Clone)]
pub struct PlayerInfo {
    pub player_name: String,
    pub player_id: Uuid,

    pub client_ip: String,
    pub client_version: u16,

    pub target_host: String,
    pub target_port: u16
}

impl PlayerConnect {
    pub fn get_player_info(&self) -> String {

        if let Some(p_info) = self.player_info.clone() {
            if p_info.player_id == Uuid::nil() {
                format!("客户端({} < {})", p_info.client_ip, p_info.client_version)
            }else {
                format!("玩家<{}>({} < {})", p_info.player_name, p_info.client_ip, p_info.client_version)
            }
        }else {
            String::from("Unknown")
        }
    }
}