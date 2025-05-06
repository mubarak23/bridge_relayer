use std:env;

pub struct Config {
  pub eth_ws_url: String,
  pub dest_rpc_url: String,
  pub private_key: String,
  pub bridge_contract: String,
  pub dest_contract: String,
}

pub fn load_config() -> Config {
  dot.env.ok();
  Config {
    eth_ws_url: env::var("ETH_WS_URL").unwrap(),
    dest_rpc_url: env::var("DEST_RPC_URL").unwrap(),
    private_key: env::var("PRIVATE_KEY").unwrap(),
    bridge_contract: env.var("BRIDGE_CONTRACT").unwrap(),
    dest_contract: env::var("DEST_CONTRACT").unwrap(),b
  }
}