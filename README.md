# CampusDAO

## Project Title

CampusDAO

## Project Description

CampusDAO is a decentralized student club governance platform built on Soroban and the Stellar blockchain. It enables students to join clubs as verified members, create governance proposals, and participate in transparent on-chain voting.

The platform demonstrates how decentralized technologies can be applied to student organizations, ensuring fairness, transparency, and trust in decision-making processes. All membership records, proposals, and voting results are securely stored on-chain and can be publicly verified.

## Project Vision

The vision of CampusDAO is to empower student organizations with decentralized governance tools that eliminate manipulation, increase transparency, and encourage active participation.

By leveraging Stellar's fast and low-cost blockchain infrastructure, CampusDAO provides a secure and efficient way for clubs and communities to manage membership and conduct voting without relying on centralized systems.

## Key Features

* **Membership Registration:** Students can join the club through wallet-authenticated transactions.
* **On-Chain Membership Records:** Membership status is stored permanently on the Stellar blockchain.
* **Proposal Creation:** Verified members can create voting proposals for club decisions.
* **Decentralized Voting:** Members can vote YES or NO on proposals.
* **Anti Double-Voting Protection:** Each member can vote only once per proposal.
* **Transparent Results:** Voting results are publicly verifiable and immutable.
* **Access Control:** Only registered members can create proposals and participate in voting.
* **Immutable Governance Records:** Proposal and voting data are stored permanently on-chain.

## Usage Instructions

1. **Deploy Contract:** Deploy the CampusDAO smart contract to Stellar Testnet.
2. **Join Club:** Register as a club member using the `join_club()` function.
3. **Verify Membership:** Check membership status using `is_member()`.
4. **Create Proposal:** Create a new voting proposal using `create_poll()`.
5. **Vote:** Cast a YES or NO vote using `vote()`.
6. **View Results:** Retrieve proposal details and results using `get_poll()`.
7. **Query Statistics:** View the total number of proposals using `get_poll_count()`.

## Future Scope

* **Membership NFT:** Upgrade membership records into Soulbound NFTs.
* **Proposal Expiration:** Add voting deadlines and automatic poll closure.
* **Multi-Option Voting:** Support voting with more than two choices.
* **Club Treasury Management:** Enable governance-controlled treasury operations.
* **Member Reputation System:** Reward active participation and contributions.
* **Frontend Dashboard:** Build a web interface for club management and voting.
* **Mobile Wallet Integration:** Support governance participation through mobile wallets.
* **Cross-Club Governance:** Allow collaboration between multiple student organizations.

## Technology Stack

* Rust and Soroban SDK for smart contract development.
* Stellar blockchain for decentralized and immutable storage.
* Cryptographic wallet authentication for secure participation.
* On-chain governance mechanisms for transparent decision-making.

## Contribution

Community contributions are welcome. Developers interested in blockchain governance, Soroban smart contracts, and decentralized community management are encouraged to fork the repository and submit pull requests.

## License

This project is licensed under the MIT License.

### Contract Detail

ID: CBQQQMEA6MV2IJZMQ4O3VDSQLR3ET5U65JLE2U4ZH4TNCAP5UUXCILZG

![alt text](image.png)

