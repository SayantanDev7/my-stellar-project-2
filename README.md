# my-stellar-project-2
A stellar contract project

# my-stellar-project
A stellar contract project

# 🎮 Gaming Skins NFT (Soroban Smart Contract)

## 📌 Project Description
Gaming Skins NFT is a decentralized smart contract built on the Stellar Soroban platform that allows players to own, trade, and manage in-game skins as Non-Fungible Tokens (NFTs).

Each skin is uniquely identified and stored on-chain, giving players true ownership of their digital gaming assets.

## 🚀 What it does
This smart contract enables:
- Minting unique gaming skins as NFTs
- Assigning ownership of skins to players
- Transferring skins between players
- Storing metadata (skin name, rarity, attributes)
- Retrieving ownership and details of skins

## ✨ Features
- 🎮 NFT-based gaming assets
- 🔐 True ownership using blockchain
- 🔄 Secure peer-to-peer transfers
- 🧩 Extensible for game integration
- ⚡ Built on Stellar Soroban for fast execution
- 🦀 Written in Rust using Soroban SDK

## 📦 Tech Stack
- Soroban SDK
- Rust
- Stellar Blockchain

## 🔗 Deployed Smart Contract Link
Gaming Skins NFT
contract id - CCVBMJN4CD43O4B4UREIZEYNB35UYLZNA7I2J2DRZS2SMTD3ZZOZM5XD

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/a0e816d4-58bb-46c1-b624-2138a2a89e8e" />


## 🛠️ How to Run Locally

```bash
# Install Soroban CLI
cargo install soroban-cli

# Build contract
soroban contract build

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/skin_nft.wasm \
  --source YOUR_IDENTITY \
  --network testnet
