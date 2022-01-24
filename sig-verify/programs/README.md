In this example we tried to implement an on-chain whitelist check using:

1. ed25519 signature verification
2. Merkle proof verifications

Both approached will now work in a Solana runtime as both operations i.e. cryptographic verification in ed25519 and SHA256 execution in the Merkle Tree solution are quite expensive for Solana runtime which reaches the max computation budge of 200,000 which turns to restrict the things we can do on-chain. 


Instead the solution is to use the allow the authority account to sign the transaction along with the final sender.
