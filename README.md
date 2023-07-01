# Substrate Node Template
## 第六课
### 1、编译
```sh
CXX=/usr/local/opt/llvm@12/bin/clang cargo build --features runtime-benchmarks --release
```

### 2、执行命令
- 基于 four_works 分支执行如下命令wasm编译
```sh
./target/release/node-template benchmark pallet \
--chain dev \
--execution wasm \
--wasm-execution compiled \
--pallet pallet_poe --extrinsic "*" \
--steps 20 --repeat 10 \
--output ./pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs
```
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/six_works/img/benchmarking1.jpg">
</div>

<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/six_works/img/benchmarking2.jpg">
</div>

- 配置对应的runtime模块中的lib文件，修改poe 模块中的lib,mock 文件，重新构建。
```sh
CXX=/usr/local/opt/llvm@12/bin/clang cargo build --features runtime-benchmarks --release
```
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/six_works/img/benchmarking3.jpg">
</div>

- 生成chain spec 文件两种格式
```sh
./target/release/node-template build-spec --chain dev > spec-dev.json
./target/release/node-template build-spec --chain=spec-dev.json --raw > spec-dev-raw.json
```
<div align="center">
  <img src="https://github.com/lizhanyang505/substrate-node-template-polkadot-v0.9.40/blob/six_works/img/chain_spec.jpg">
</div>
