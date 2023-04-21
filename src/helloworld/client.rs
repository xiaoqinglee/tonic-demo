pub mod helloworld_api {
    tonic::include_proto!("helloworld");
}
use helloworld_api::{greeter_client, greeter_server, HelloRequest, HelloReply};

fn main() {
    println!("hello")
}