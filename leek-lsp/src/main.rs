#[macro_use]
extern crate serde;

mod lsp;

#[macro_use]
extern crate jsonrpc_derive;

use jsonrpc_core::{Result as RPCResult, IoHandler};
use jsonrpc_stdio_server::ServerBuilder;
use crate::lsp::{LSP, InitalizeParams, InitializeResult, ServerCapabilities};
use std::process::exit;

#[derive(Copy, Clone, Default, Debug)]
pub struct LeekLSP;

impl LSP for LeekLSP {
    fn initialize(_params: InitalizeParams<(), ()>) -> RPCResult<InitializeResult<()>> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                ..Default::default()
            }
        })
    }

    fn shutdown() -> RPCResult<()> {
        println!("Shutdown requested");
        Ok(())
    }

    fn exit() -> RPCResult<()> {
        println!("Exit requested");
        exit(0)
    }
}

pub fn main() {
    let mut io = IoHandler::new();
    io.extend_with(LeekLSP.to_delegate());

    ServerBuilder::new(io)
        .build();
}
