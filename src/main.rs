use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use rpc::api::register_rpc;

const REGISTER_RPC: &'static str = "register_cota_cells";

fn main() {
    let mut io = IoHandler::default();
    io.add_method(REGISTER_RPC, move |params: Params| register_rpc(params));

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3050".parse().unwrap())
        .unwrap();

    server.wait();
}
