# SariSplit
**Digital tab management for Philippine sari-sari stores on Stellar**
## Problem
Sari-sari store owners in the Philippines lose ₱200–500/month tracking informal credit ("utang") given to customers who pay in irregular cash installments.
## Solution
SariSplit creates on-chain tabs for each customer. Store owners create tabs, customers pay via USDC QR codes, and balances update in real-time with full transparency.
## Timeline
- **Week 1:** Core contract + CLI testing
- **Week 2:** Mobile web frontend with QR generation
- **Week 3:** Testnet deployment + user testing
- **Week 4:** Polish and demo prep
## Stellar Features Used
- USDC transfers (payment settlement)
- Soroban smart contracts (tab state management)
- Trustlines (USDC asset trust)
## Vision
Scale to 1M+ sari-sari stores across SEA, reducing informal credit losses and building financial history for small merchants.
## Prerequisites
- Rust 1.74+
- Soroban CLI v20.0.0+
- Stellar testnet account with XLM
## Build
```bash
soroban contract build
Test
bash


cargo test
Deploy to Testnet
bash


soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sari_split.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet
Sample Invocation
bash


# Initialize contract
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <OWNER_SECRET> \
  --network testnet \
  -- initialize \
  --owner <OWNER_ADDRESS>
# Create a tab
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <OWNER_SECRET> \
  --network testnet \
  -- create_tab \
  --customer <CUSTOMER_ADDRESS> \
  --initial_balance 10000000
# Customer pays
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <CUSTOMER_SECRET> \
  --network testnet \
  -- pay_tab \
  --customer <CUSTOMER_ADDRESS> \
  --amount 4000000
License
MIT

✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/2c964d8bcd9c38abbe557553392a125a358bb585bfba6f835ac9e5fb81ebe49b
🔗 https://lab.stellar.org/r/testnet/contract/CCYNYYKNEDQXTXYDO6OMSZU5FYBFDMQUSL72THCGPG5WTHTTK7XMEUDO
✅ Deployed!
CCYNYYKNEDQXTXYDO6OMSZU5FYBFDMQUSL72THCGPG5WTHTTK7XMEUDO