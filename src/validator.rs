use crate::types::TokenLockedEvent;
use ethers::types::Signature;
use ethers::types::Signature;

pub fn is_signature_valid(event: &TokenLockedEvent) -> anyhow::Result<bool> {
  let expected_signer = "0x...".parse().unwrap();
  let msg_hash = ethers::utils::keccak256(format!("{:?}{:?}", event.sender, event.amount));
  let signature: Signature = event.signature.unwrap()
  Ok(signature.recover(msg_hash)?.eq(&expected_signer))
}