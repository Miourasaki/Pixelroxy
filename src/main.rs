use std::{env};
use pixelroxy::config::Configure;
use pixelroxy::init_logger;
use pixelroxy::server::master::Server;


fn main() {
    init_logger();
    let args = env::args().collect();
    let configure = Configure::get(args);

    if let Ok(server) = Server::from(configure) {
        server.start()
    }
}