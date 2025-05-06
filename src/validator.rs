use crate::types::TokenLockedEvent;
use ethers::types::Signature;


pub fn is_signature_valid(event: &TokenLockedEvent) -> anyhow::Result<bool> {
  let expected_signer = "0x...".parse().unwrap(); // to be replaced with an actual Ethereum address (like "0x123abc...").
  let msg_hash = ethers::utils::keccak256(format!("{:?}{:?}", event.sender, event.amount)); // will optimize using ABI-encoding
  let signature: Signature = event.signature.clone().ok_or_else(||, anyhow::anyhow!("Missing signature in TokenLockedEvent"))?
  Ok(signature.recover(msg_hash)?.eq(&expected_signer))
}