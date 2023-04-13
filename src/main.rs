use zmq::{proxy_with_capture, Socket};

fn main() {
    let context = zmq::Context::new();
    let mut router = context.socket(zmq::ROUTER).unwrap();
    router.bind("tcp://*:9951").unwrap();
    set_keep_alives(&mut router);
    let mut dealer = context.socket(zmq::DEALER).unwrap();
    dealer.bind("tcp://*:9950").unwrap();
    set_keep_alives(&mut dealer);
    let mut publish = context.socket(zmq::PUB).unwrap();
    publish.bind("tcp://*:9960").unwrap();
    set_keep_alives(&mut publish);
    
    println!("Running the proxy. 🚀🚀🚀");
    proxy_with_capture(&mut router, &mut dealer, &mut publish).expect("Failed to create proxy.");
}

fn set_keep_alives(sock: &mut Socket) {
    match sock.set_tcp_keepalive(1) {
        Ok(_) => {},
        Err(e) => {println!("set_tcp_keepalive: {}", e)},
    } 
    match sock.set_tcp_keepalive_cnt(i32::MAX) {
        Ok(_) => {},
        Err(e) => {println!("set_tcp_keepalive_cnt: {}", e)},
    } 
    match sock.set_tcp_keepalive_idle(i32::MAX) {
        Ok(_) => {},
        Err(e) => {println!("set_tcp_keepalive_idle: {}", e)},
    } 
    match sock.set_tcp_keepalive_intvl(i32::MAX) {
        Ok(_) => {},
        Err(e) => {println!("set_tcp_keepalive_intvl: {}", e)},
    }
}