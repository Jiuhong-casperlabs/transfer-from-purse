# transfer-from-purse
The main source includes the following steps
## 1. create purse:
### This step created a new purse under the account context
```
let second_purse = system::create_purse();
```
## 2. fund purse
### This step transfers amount from account's main purse to the newly created purse.
```
let source: URef = account::get_main_purse();
...
system::transfer_from_purse_to_purse(source, second_purse, amount, None).unwrap_or_revert();
```
## 3. store purse into contract named_keys
### This step stores newly created purse into the contract's named_keys
```
let mut named_keys: BTreeMap<String, Key> = BTreeMap::new();

//store purse into contract's named_keys
named_keys.insert(String::from("second_purse"), second_purse.into());
```
## 4. entry point
   - get target account hash
   ```
   let account_hash_key: Key = runtime::get_named_arg("account_hash");
    let target_account_hash = account_hash_key.into_account().unwrap();
   ```
   - transfer amount from created purse to target public key
   ```
   system::transfer_from_purse_to_account(source_purse, target_account_hash, amount, None)
        .unwrap_or_revert();
   ```

## Command example
### 1. install contract
```
casper-client put-deploy \
--chain-name casper-test \
--node-address http://94.130.10.55:7777 \
--payment-amount 20000000000 \
--session-path /home/jh/mywork/transfer-from-purse/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--session-arg "amount:U512='9000000000'" \
--secret-key /home/jh/keys/test2/secret_key.pem 
```

### 2. call entrypoint `transfer_amount`
```
casper-client put-deploy --chain-name casper-test \
--node-address http://94.130.10.55:7777 \
--secret-key /home/jh/keys/test2/secret_key.pem \
--session-hash "hash-4f3de8e812f8bfee3e088461b325b5136d69307fc62aafa034298e16b818e332" \
--session-entry-point "transfer_amount"  \
--session-arg "account_hash:key='account-hash-d9758b25962f4cba82ba0047389af97a70acb7df43b391f9ffb293801bea5061'" \
--payment-amount 3000000000
```

<!-- pk:010e31a03ea026a8e375653573e0120c8cb96699e6c9721ae1ea98f896e6576ac3 -->