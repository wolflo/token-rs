Playing with rust and [ethers-rs](https://github.com/gakonst/ethers-rs) for writing smart contract tests.
The contracts are just various tokens from [OpenZeppelin](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/token/ERC20).

# Usage

You can compile the contracts with:
```
cargo build
```
This will use your local `solc` install.

Compile contracts and run the tests:
```
cargo test -- --nocapture
```
