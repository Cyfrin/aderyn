use aderyn_driver::{
    detection_modules::high::{
        arbitrary_transfer_from::ArbitraryTransferFromDetector,
        delegate_call_in_loop::DelegateCallInLoopDetector,
    },
    detector::Detector,
};

struct Contestant {
    id: String,
    detector: Box<dyn Detector>,
}

fn main() {
    let contestants: Vec<Box<dyn Detector>> = vec![
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<DelegateCallInLoopDetector>::default(),
    ];

    println!("Hello, world!");
}
