mod structs;

use jsonrpc_core::Result as RPCResult;
pub use structs::*;

#[rpc]
pub trait LSP {
    #[name="initialize"]
    fn initialize(params: InitalizeParams<(), ()>) -> RPCResult<InitializeResult>;
    fn shutdown() -> RPCResult<()>;
    fn exit() -> RPCResult<()>;
}