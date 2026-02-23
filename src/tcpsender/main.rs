// use std::net::{SocketAddr, TcpListener};
// use httpfromtcp::utils::get_lines_channel;


// fn main() -> std::io::Result<()> {
//     let address = SocketAddr::from(([127, 0, 0, 1], 8080));

//     let listener = TcpListener::bind(&address)?;

//     for stream in listener.incoming() {

//         let stream = stream?;
//         println!("connection accepted");
//         get_lines_channel(stream)?;
//         println!("connection closed");
//     }

//     Ok(())
// }

struct Paypal {
    amount: usize,
    payer: String,
    receiver: String
}

struct Razorpay {
    amount: usize,
    payer: String,
    receiver: String
}

trait PaymentProcessing {
    fn process_payment(&self);
}

impl PaymentProcessing for Paypal {
    fn process_payment(&self) {
        todo!()
    }
}

impl PaymentProcessing for Razorpay {
    fn process_payment(&self) {
        todo!()
    }
}

impl Paypal {
    fn new(&mut self, data: &[u8]) {
        todo!()
    }

    fn something_unique_by_paypal() {
        todo!()
    }
}

impl Razorpay {
    fn new(&mut self, data: &[u8]) {
        todo!()
    }

    fn something_unique_by_razorpayl() {
        todo!()
    }
}



fn main(){
    println!("hello");
}

