# arkworks_icp24
This repository uses arkworks' ark_groth16 to generate a Proof on the front end and verify it on the Canister Smart Contract (Internet Computer∞).

![app](/src/arkworks_icp24_frontend/assets/Screenshot%202023-04-17%20at%2011.34.34.png)

## Important File
```
src/arkworks_icp24_backend/src/lib.rs           → Verifying a ZK Proof
src/arkworks_icp24_frontend/rust_zkp/src/lib.rs → Generating a ZK Proof
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```