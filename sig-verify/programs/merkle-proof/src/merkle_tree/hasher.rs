use anchor_lang::prelude::*;
use std::convert::TryFrom;
use sha2::{digest::FixedOutput, Digest, Sha256};

#[derive(AnchorSerialize, AnchorDeserialize)]
struct Leaf<'a> {
  data: &'a [u8],
}

pub trait Hasher: Clone {
  type Hash: Copy + PartialEq + Into<Vec<u8>> + TryFrom<Vec<u8>> + core::fmt::Debug;

  fn hash(data: &[u8]) -> Self::Hash;

  fn concat_and_hash(left: &Self::Hash, right: &Self::Hash) -> Self::Hash {
    let mut concatenated: Vec<u8> = (*left).into();

    let mut right_node_clone: Vec<u8> = (*right).into();
    concatenated.append(&mut right_node_clone);
    Self::hash(&concatenated)
  }
}

pub struct MerkleProof<'a, T: Hasher> {
  path: &'a [T::Hash]
}

impl <'a, T: Hasher> MerkleProof<'a, T> {
  pub fn new(path: &'a [T::Hash]) -> Self {
    Self {path} 
  }

  pub fn verify(&self, merkle_root: T::Hash) -> Result<(), String> {
    let result = self.path
      .to_vec()
      .into_iter()
      .reduce(|acc, next| T::concat_and_hash(&acc, &next));

    msg!("{:#?}", result);
    match result {
      Some(hash) if hash == merkle_root => Ok(()),
      _ => Err("Empty path".to_owned())
    }
  }
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
    let mut hasher = Sha256::new();

    hasher.update(&Self::encode(data));
    <[u8; 32]>::from(hasher.finalize_fixed())
  }
}
