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
--session-hash "hash-01efcdd4aada8f161802c6f603544966d36c7c72c636da323346eb5e397c544c" \
--session-entry-point "transfer_amount"  \
--session-arg "account_hash:key='account-hash-d9758b25962f4cba82ba0047389af97a70acb7df43b391f9ffb293801bea5061'" \
--payment-amount 3000000000
```