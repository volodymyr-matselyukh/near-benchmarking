#!/bin/sh

WITH_INIT="${1:-nothing}"

if [[ "$WITH_INIT" == "nothing" ]]; then
    echo "Should the initialization of contract be performed [y/N]:"
    read WITH_INIT
fi

with_init_choice_lowercase_string=$(echo "$WITH_INIT" | tr '[:upper:]' '[:lower:]')

if [[ 
  "$with_init_choice_lowercase_string" != "nothing" 
  && "$with_init_choice_lowercase_string" != ""
  && "$with_init_choice_lowercase_string" != "n" 
  && "$with_init_choice_lowercase_string" != "no" ]]; then

  echo "----------------Deploying the contract with initialization----------------"
else 
  echo "----------------Deploying the contract----------------"
fi

./build.sh

near contract deploy unused-name.testnet use-file ../target/wasm32-unknown-unknown/release/benchmarking.wasm without-init-call network-config testnet sign-with-keychain send


if [[ 
  "$with_init_choice_lowercase_string" != "nothing" 
  && "$with_init_choice_lowercase_string" != ""
  && "$with_init_choice_lowercase_string" != "n" 
  && "$with_init_choice_lowercase_string" != "no" ]]; then

    echo "----------------Initializing the contract----------------"

    near contract call-function as-transaction unused-name.testnet new json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as unused-name.testnet network-config testnet sign-with-keychain send

    echo "----------------Registering contract in usdt.fakes.testnet----------------"

    near contract call-function as-transaction usdt.fakes.testnet storage_deposit json-args '{"account_id": "unused-name.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as unused-name.testnet network-config testnet sign-with-keychain send
fi
