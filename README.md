# Crypto Price Checker

A simple command-line tool written in Rust to check cryptocurrency prices using the CoinGecko API. This project was created as a learning exercise for Rust.

## Features

- Fetch real-time prices for any cryptocurrency supported by CoinGecko.
- Support for multiple target currencies (defaults to USD).
- Built with modern Rust libraries:
    - `clap` for robust CLI argument parsing.
    - `reqwest` for asynchronous HTTP requests.
    - `tokio` for the asynchronous runtime.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd crypto-price-checker
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

You can run the tool using `cargo run` or by executing the compiled binary.

### Using Cargo

```bash
# Get the price of Bitcoin in USD (default)
cargo run -- --token bitcoin

# Get the price of Ethereum in EUR
cargo run -- --token ethereum --currency eur
```

### Using the Binary

After building with `--release`, the binary is located in `target/release/`.

```bash
./target/release/crypto-price-checker --token solana --currency jpy
```

### Options

| Short | Long | Description | Default |
| :--- | :--- | :--- | :--- |
| `-t` | `--token` | The ID of the cryptocurrency (e.g., `bitcoin`, `ethereum`) | **Required** |
| `-c` | `--currency` | The target currency (e.g., `usd`, `eur`, `gbp`) | `usd` |
| `-h` | `--help` | Print help information | - |
| `-V` | `--version` | Print version information | - |

## API Reference

This tool uses the [CoinGecko Simple Price API](https://www.coingecko.com/en/api/documentation). Note that the API has rate limits for the public tier.

## License

This project is open-source and available under the MIT License.
