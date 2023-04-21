git submodule add https://github.com/googleapis/googleapis proto/googleapis
git submodule update --remote

cargo build

cargo run --bin helloworld-client
