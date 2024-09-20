echo ">> Feeding near 100_000"

max=100
for (( i=0; i <= $max; ++i ))
do
    near contract call-function as-transaction unused-name.testnet fill_near_100_000 json-args '{}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as unused-name.testnet network-config testnet sign-with-keychain send
done