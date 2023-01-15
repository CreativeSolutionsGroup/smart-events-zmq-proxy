use zmq::proxy_with_capture;

fn main() {
    let context = zmq::Context::new();
    let mut router = context.socket(zmq::ROUTER).unwrap();
    router.bind("tcp://*:9951").unwrap();
    let mut dealer = context.socket(zmq::DEALER).unwrap();
    dealer.bind("tcp://*:9950").unwrap();
    let mut push = context.socket(zmq::PAIR).unwrap();
    push.bind("tcp://*:9952").unwrap();
    
    println!("Running the proxy. 🚀🚀🚀");
    proxy_with_capture(&mut router, &mut dealer, &mut push).expect("Failed to create proxy.");
}