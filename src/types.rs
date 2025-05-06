use ethers::types::{Address, U256, Signature};

#[derive(Debug, Clone)]
pub struct TokenLockedEvent {
  pub sender: Address,
  pub amount: u256,
  pub recipient: Address,
  pub signature: Option<Signature>
}