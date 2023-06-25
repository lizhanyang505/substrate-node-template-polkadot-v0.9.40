
# 第五课
## 1.环境初始化
```sh
cargo install dylint-link
brew update
brew reinstall openssl@1.1
brew install binaryen
cargo install cargo-contract
rustup toolchain install nightly-2023-03-18
rustup show
rustup toolchain install nightly-2023-03-18-x86_64-apple-darwin
rustup toolchain add nightly-2023-03-18-x86_64-apple-darwin
rustup default nightly-2023-03-18-x86_64-apple-darwin
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-18-x86_64-apple-darwin
cargo contract new flipper
cargo contract build
cargo install cargo-expand



cargo contract new erc20
cargo contract build
cargo expand > out.rs

```
## alice 给bob_stash 授权121
- alice授权给bob_stash之后的balance
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/five_works/img/approval_bob_stash.jpg">
</div>

- 执行授权后，alice balance
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/five_works/img/approval_balnace_alice.jpg">
</div>

## alice 给bob 转账100
- alice 转账to bob
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/five_works/img/alice_transfer_to_bob.jpg">
</div>

- bob balance
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/five_works/img/bob_balance.jpg">
</div>

- alice 最后balance
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/five_works/img/alice_balance.jpg">
</div>






