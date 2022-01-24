use anchor_lang::prelude::*;
use rs_merkle::{
  Hasher,
  algorithms::Sha256
};

#[derive(AnchorSerialize, AnchorDeserialize)]
struct Leaf<'a> {
  data: &'a [u8],
}

#[derive(Clone)]
pub struct BoshHasher;

impl BoshHasher {
  fn encode(data: &[u8])-> Vec<u8>  {
    let raw_message = Leaf {data};
    let mut message: Vec<u8> = Vec::new();
    raw_message.serialize(&mut message).unwrap();

    message
  }
}

impl Hasher for BoshHasher {
  type Hash = [u8; 32];

  fn hash(data: &[u8]) -> [u8; 32] {
    Sha256::hash(&Self::encode(data))
  }
}
