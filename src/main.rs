extern crate yourcloud;
use yourcloud::Log;

fn main() {
    let mut log = Log::new("Hello World");
    log.create("/foobar.txt");
    println!("{}", log);
}
