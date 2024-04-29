# ClosedCred Micro Investments Smart Contract

This is a Rust smart contract for the Stellar blockchain, designed to enable micro-investments and various investment tools for the ClosedCred project. It allows users to save small amounts through round-ups, daily deposits, and lump-sum investments, and also provides the ability to invest in stocks, mutual funds, and cryptocurrencies.

## Features

- **Round-Up Investments**: Automatically rounds up transaction amounts based on a configurable percentage (defaulting to 1%) and adds the round-up amount to the user's balance.
- **Daily Deposits**: Automatically deducts a daily deposit amount (defaulting to ₹100) from the user's account and adds it to their balance.
- **Lump-Sum Investments**: Allows users to deposit custom lump-sum amounts into their balance.
- **Investment Tools**: Provides a framework for enabling various investment tools, such as stocks, mutual funds, and cryptocurrencies.
- **Investment Function**: Allows users to invest their balance in the enabled investment tools, with proper balance verification and access control.

## Getting Started

### Prerequisites

- Rust toolchain (https://www.rust-lang.org/tools/install)
- Soroban CLI (`cargo install --git https://github.com/stellar/rs-soroban-sdk soroban-cli`)

### Building the Contract

1. Clone or download the project repository.
2. Navigate to the project directory.
3. Run `soroban-cli build` to compile the contract into a WebAssembly (Wasm) binary.

### Deploying the Contract

1. Set the `SOROBAN_RPC` environment variable to connect to the desired Stellar network (e.g., testnet or mainnet).
2. Run `soroban-cli deploy ./target/wasm32-unknown-unknown/release/your_contract.wasm --wasm --rpc-node '<network_rpc_url>'` to deploy the contract.
3. Note the contract ID printed in the output.

### Interacting with the Contract

You can invoke the contract's functions using the `soroban-cli contract invoke` command:

- `soroban-cli contract invoke <contract_id> initialize --src ./target/wasm32-unknown-unknown/release/your_contract.wasm`
- `soroban-cli contract invoke <contract_id> round_up <amount> --src ./target/wasm32-unknown-unknown/release/your_contract.wasm`
- `soroban-cli contract invoke <contract_id> daily_deposit --src ./target/wasm32-unknown-unknown/release/your_contract.wasm`
- `soroban-cli contract invoke <contract_id> lumpsum_deposit <amount> --src ./target/wasm32-unknown-unknown/release/your_contract.wasm`
- `soroban-cli contract invoke <contract_id> invest <investment_type> <amount> --src ./target/wasm32-unknown-unknown/release/your_contract.wasm`

Replace `<contract_id>` with the actual ID of your deployed contract, and `<amount>` and `<investment_type>` with the desired values.

### Viewing Contract Data and Logs

To view the contract's data and logs, run the following commands:

- `soroban-cli contract data <contract_id>`
- `soroban-cli contract logs <contract_id>`

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any bug fixes, improvements, or new features.

## License

This project is unlicensed.