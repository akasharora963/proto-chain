## Proto Chain (A simple representation of blockchain)

A simple representation of a blockchain implemented in Rust.

## Overview

This project demonstrates a basic blockchain structure, focusing on core concepts such as blocks, chains, and consensus mechanisms. It's designed for educational purposes to help understand how blockchains function under the hood.

## Features

- **Block Creation**: Each block contains data, a hash, and a reference to the previous block's hash.
- **Chain Validation**: Ensures the integrity of the blockchain by validating hashes and the sequence of blocks.
- **Proof of Work**: Implements a simple consensus mechanism to add new blocks to the chain.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Ensure you have the latest stable version installed)

### Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/akasharora963/proto-chain.git
   cd proto-chain
   ```

2. **Build the Project**:

   ```bash
   cargo build --release
   ```

3. **Run the Application**:

   ```bash
   cargo run
   ```

## Usage

After running the application, the program will:

1. Initialize a new blockchain.
2. Add a set number of blocks to the chain.
3. Display each block's details, including its index, data, hash, and previous hash.

This simulation showcases how blocks are added to the blockchain and how each block is linked to its predecessor, ensuring data integrity.


## Acknowledgments

- Inspired by various educational resources on blockchain technology.
- Thanks to the Rust community for their support and contributions.

---

*Note: This project is for educational purposes and is not intended for production use.*


