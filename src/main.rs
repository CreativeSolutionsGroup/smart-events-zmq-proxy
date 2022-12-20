use zmq::proxy;

fn main() {
    let context = zmq::Context::new();
    let xpub = context.socket(zmq::XPUB).unwrap();
    xpub.bind("tcp://*:9951").unwrap();
    let xsub = context.socket(zmq::XSUB).unwrap();
    xsub.bind("tcp://*:9950").unwrap();

    println!("Running the proxy. 🚀🚀🚀");
    proxy(&xpub, &xsub).expect("Failed to create proxy.");
}
