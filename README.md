# Soroban-Math

![Crates.io](https://img.shields.io/crates/v/soroban-math)
![Crates.io](https://img.shields.io/crates/l/soroban-math)
![Crates.io](https://img.shields.io/crates/d/soroban-math)
![crate publish workflow](https://github.com/rahul-soshte/soroban-math/actions/workflows/publish.yml/badge.svg)

A high-precision fixed-point math library for Soroban smart contracts on the Stellar blockchain.

**This project aims to empower Soroban developers with precise mathematical operations and advanced functions while minimizing precision loss. It is currently under development and subject to enhancements and optimizations.**


## Quickstart

Add this to your Cargo.toml:

```toml
[dependencies]
soroban-math = "0.1.7"
```

And this to your code:

```rust
use soroban_math::*;
```

## Features
1. High-Precision Arithmetic: Perform calculations with numbers using the i128 type, ensuring precision and accuracy.
2. Advanced Math Functions: Access advanced mathematical functions like logarithms, exponentiation, and more.
3. Ergonomic API: An intuitive API for easy integration into Soroban smart contracts.
4. Minimized Rounding Losses: Specialized techniques to reduce rounding errors during calculations.
5. Security: Audit-ready code to ensure the safety and correctness of calculations.

## Use Cases

Soroban-Math is essential for any Soroban-based project that requires high-precision mathematical calculations, for example:

1. DEX protocols
2. Stablecoins 
3. Lending Protocols

## Authors

Rahul Soshte ([Twitter](https://twitter.com/RahulSoshte))