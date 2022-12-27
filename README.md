<h1 style="text-align: center;"><a href="https://casperlabs.io/"><img src="https://raw.githubusercontent.com/gRoussac/casper-deployer-hackathon-oct-2022/dev/www/apps/frontend/src/assets/logo-dark.svg" width="120" style="position: relative; top:5px" alt="Casper"></a> <b>Inventory Management</b>

# **[CasperLabs](https://casperlabs.io/)** Blockchain-based inventory management system

# 📔 **Project**

#### This projects' purpose is to serve as a baseline proof-of-concept of an inventory management system on the <a href="https://casperlabs.io/"><img  style="position: relative; top:3px" alt="Casper" src="https://user-images.githubusercontent.com/3099551/197350250-b9d5852b-44a6-45bb-a227-e12d6d4166c9.jpg" height="20" width="20" alt="Casper"/> Casper Blockchain</a> with pre-composed smart contracts and provide small examples of this use case in action, including queries done with the [Casper Client CLI](https://github.com/casper-ecosystem/casper-client-rs) and example front-end solutions.

🏭 **Tech Stack**

- **Blockchain ⛓️**

  <img alt="Casper" width="100" src="https://user-images.githubusercontent.com/3099551/197353903-d2bdc636-9dcd-4e84-8ba7-6887d59f6b8a.png"/>

- **Frontend**

  Coming Soon

- **Backend**

  <img alt="Rust" src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"/>
  <img alt="WEBASSEMBLY" src="https://camo.githubusercontent.com/9734598c5ee062706c931512b7572b5675274ee5a728b9afddfe0d4cdd1ba82d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f7374796c653d666f722d7468652d6261646765266d6573736167653d576562417373656d626c7926636f6c6f723d363534464630266c6f676f3d576562417373656d626c79266c6f676f436f6c6f723d464646464646266c6162656c3d"/>

## Smart contracts

Smart contracts are implemented in [Rust](https://www.rust-lang.org/) + [Casper smart contract crate](https://docs.rs/casper-contract/latest/casper_contract/).

# 🛣️ Roadmap / Todo / Tofix

- MVP
    - [✓] Develop contract that creates a dictionary of items in inventory 🏬
    - [✓] Develop contract function that increments a given item in inventory by a specified amount (indicating shipment received) 🚚
    - [✓] Develop contract function that decrements a given item in inventory by a specified amount (indicating item(s) sold) 🚚
    - [✓] Develop contract function that adds a new item to the inventory system.
- Front-end
    - [] Develop an "order" interface to "sell" items.
    - [] Develop a "shipments" interface to "receive" items.
    - [] Develop a "management" interface to "add" newly stocked items.
    - [] Implement an example front-end marketplace use case for inventory management.
- Back-end
    - [] Fine-tune contract functions to optimize/lower cost of contract calls.
    - [] Integrate signer for shipments and management portions with Admin key access management
    - [] Integrate signer for orders interface as a potential payment option
- Housekeeping
    - [] Document contract code
    - [] Build out unit tests
    - [] Simplify code

### ❓Have questions?

Go to the `#dev-discussion` channel [on Discord](https://discord.gg/casperblockchain)

### 🪦 Errors ?

If you see any typos or errors you can edit the code directly on GitHub and raise a Pull Request on the `feature` branch, thank you in advance!