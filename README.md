# STUDENTS MANAGER - ICP DApp Deployment (Rust + React)
## Prerequisites
- Node.js & npm
- Rust (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)
- DFX SDK (dfxvm install and dfxvm use latest)
- Internet Computer setup (dfx, cargo, etc.)
## Clone the Repo
```bash
git clone https://github.com/shivaraaman/Dapp_ICP.git
cd Dapp_ICP
```
## Install Frontend Dependencies
```bash
cd src/Dapp-icp-frontend
npm install
cd ../../
```
## Start Local Internet Computer
```bash
dfx start --clean
```
Then Open new Terminal
## Deploy Canisters
```bash
dfx deploy
```
## Access the Frontend
Open in your browser:
```
http://<frontend_canister_id>.localhost:xxxx/
```
Example:
```
http://u6s2n-gx777-77774-qaaba-cai.localhost:xxxx/
```
## Example Backend Call
```bash
dfx canister call Dapp-icp-backend add_student '("Siva", 450, 5)'
dfx canister call Dapp-icp-backend get_students
```
## What It Does
- Backend: Rust-based canister to store and process student data.
- Frontend: React app to interact with the backend via Candid interface.
- Grade logic:
 - A: 90+
 - B: 75-89
 - C: 60-74
 - D: below 60
