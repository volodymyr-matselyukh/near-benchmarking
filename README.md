# benchmarking
This project is created for comparing the performance of near collections versus native collections.

# check_native_1000
`
near contract call-function as-transaction woozy-loaf.testnet check_native_1000 json-args '{"value": 1}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as woozy-loaf.testnet network-config testnet sign-with-keychain send
`


# fill_near_1000
`
near contract call-function as-transaction woozy-loaf.testnet fill_near_1000 json-args '{}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as woozy-loaf.testnet network-config testnet sign-with-keychain send
`

# fill_native_1000
`
near contract call-function as-transaction woozy-loaf.testnet fill_native_1000 json-args '{}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as woozy-loaf.testnet network-config testnet sign-with-keychain send
`

# fill_near_10_000
`
near contract call-function as-transaction woozy-loaf.testnet fill_near_10_000 json-args '{}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as woozy-loaf.testnet network-config testnet sign-with-keychain send
`

# fill_native_10_000
`
near contract call-function as-transaction woozy-loaf.testnet fill_native_10_000 json-args '{}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as woozy-loaf.testnet network-config testnet sign-with-keychain send
`

# check_native_10_000
`
near contract call-function as-transaction woozy-loaf.testnet check_native_10_000 json-args '{}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as woozy-loaf.testnet network-config testnet sign-with-keychain send
`

# check_near_10_000
`
near contract call-function as-transaction woozy-loaf.testnet check_near_10_000 json-args '{}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as woozy-loaf.testnet network-config testnet sign-with-keychain send
`