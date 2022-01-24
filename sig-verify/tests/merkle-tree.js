import {MerkleTree} from 'merkletreejs'
import borsh from 'borsh'
import crypto from 'crypto'

// remove the 0x from the value
const keccak256 = val => crypto
    .createHash('sha256')
    .update(val)
    .digest('hex')

class Entry {
  constructor(sender) {
    this.sender = sender.toString()
  }
}

const type = {
  "kind": "struct",
  "fields": [
    ['sender', ['string']]
  ]
}

const schema = new Map([[Entry, type]]);

const createLeaf = account => borsh.serialize(schema, new Entry(account))

const preProcess = accounts => {
  const result = []

  for (let i = 0; i < accounts.length; i++) {
    result.push(createLeaf(accounts[i]))
  }

  return result
}

export const createTree = accounts => {
  const leaves = preProcess(accounts)
  return new MerkleTree(leaves, keccak256, {sortPairs: true, hashLeaves: true})
}

export const getRoot = tree => Uint8Array.from(tree.getRoot())
export const getProof = (tree, leaf) => tree.getHexProof(keccak256(leaf))
