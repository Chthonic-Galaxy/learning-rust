use network::server;
use network::vzlom;

mod network;

pub fn call_f() {
    server::hack_nsa();
}

fn main() {
    call_f();

    vzlom();
}
