# STUDENTS MANAGER - ICP DApp Deployment (Rust + React)
## Prerequisites
- Node.js & npm
- Rust (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)
- DFX SDK (dfxvm install and dfxvm use latest)
- Internet Computer setup (dfx, cargo, etc.)

- OPEN YOUR DFX ENVIRONMENT IN TERMINAL USING UBUNTU OR OTHER
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
HERE YOU CAN OPEN IN BOTH WAYS. WEB AND IN TERMINAL
## Access the Web Frontend
Open in your browser:
```
http://<frontend_canister_id>.localhost:xxxx/
```
Example:
```
http://u6s2n-gx777-77774-qaaba-cai.localhost:xxxx/
```
## Access the Terminal 
```bash
dfx canister call Dapp-icp-backend add_student '("Siva", 450, 5)'
dfx canister call Dapp-icp-backend get_students
```
## 🧪 Screenshots & Demo
 DFX Start
![start](Dapp_icp/Screenshot%202025-06-23%20023532.png)
Deploy Canisters
![deploy](Dapp_icp/Screenshot%202025-06-23%20023638.png)
Get Canister Links
![links](Dapp_icp/Screenshot%202025-06-23%20023720.png)
Frontend Links
![Frontend links](Dapp_icp/Screenshot%202025-06-23%20023845.png)
Frontend UI
![Frontend UI](Dapp_icp/Screenshot%202025-06-23%20023911.png)
Input a Student
![Frontend input](Dapp_icp/Screenshot%202025-06-23%20023947.png)
View All Students
![Frontend ls of students](Dapp_icp/Screenshot%202025-06-23%20024017.png)
Terminal Input
![terminal Input](Dapp_icp/Screenshot%202025-06-23%20024559.png)
Terminal List of Students
![Terminal list](Dapp_icp/Screenshot%202025-06-23%20024622.png)
## What It Does
- Backend: Rust-based canister to store and process student data.
- Frontend: React app to interact with the backend via Candid interface.
- Grade logic:
 - A: 90+
 - B: 75-89
 - C: 60-74
 - D: below 60
