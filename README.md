# Soroban-Math

![Crates.io](https://img.shields.io/crates/v/soroban-math)
![Crates.io](https://img.shields.io/crates/l/soroban-math)
![Crates.io](https://img.shields.io/crates/d/soroban-math)
![crate publish workflow](https://github.com/rahul-soshte/soroban-math/actions/workflows/publish.yml/badge.svg)

A high-precision fixed-point math library for Soroban smart contracts on the Stellar blockchain.

**This project aims to empower Soroban developers with precise mathematical operations and advanced functions while minimizing precision loss. It is currently under development and not functional and subject to enhancements and optimizations.**


## Quickstart

Add this to your Cargo.toml:

```toml
[dependencies]
soroban-math = "0.2.8"
```

And this to your code:

```rust
use soroban_math::*;
```

## Features
1. High-Precision Arithmetic: Perform calculations with numbers using the i128/I256 type, ensuring precision and accuracy.
2. Advanced Math Functions: Access advanced mathematical functions like logarithms, exponentiation, and more.
3. Ergonomic API: An intuitive API for easy integration into Soroban smart contracts.
4. Minimized Rounding Losses: Specialized techniques to reduce rounding errors during calculations.

## Some Notes

I am monitoring and maintaining this repo closely. Writing test cases. I cannot ask you to assume that the library is 100% safe to use in all scenarios. If you need my consultation while integrating this library and implementing it as safely as possible in your project / hands-on custom implementation, I sure can do it, of course that will be come with some price tag, please DM me on [Twitter](https://twitter.com/RahulSoshte) or mail me here, rahul.soshte47@gmail.com.

## Getting Help

Join the [discord server](https://discord.gg/Dy7BXxUJHC) to chat with the community!

## Use Cases

Soroban-Math is essential for any Soroban-based project that requires high-precision mathematical calculations, for example:

1. DEX protocols
2. Stablecoins 
3. Lending Protocols

## Authors

Rahul Soshte ([Twitter](https://twitter.com/RahulSoshte))