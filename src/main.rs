use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use rpc::api::generate_registry_cota_smt;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("register_cota_cells", move |params: Params| {
        generate_registry_cota_smt(params)
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
