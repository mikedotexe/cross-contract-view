# Cross-contract view call

Note, there's a whitelist contract where `mike.testnet` is approved. For more information, visit the [whitelist contract](https://github.com/near/core-contracts/tree/master/whitelist).

./build.sh
near deploy cross-contract-view.demo.testnet --wasmFile res/cross_contract_view.wasm
near call cross-contract-view.demo.testnet xcc_use_promise_result --accountId mike.testnet --gas 200000000000000
near call cross-contract-view.demo.testnet xcc_use_arg_macro --accountId mike.testnet --gas 200000000000000