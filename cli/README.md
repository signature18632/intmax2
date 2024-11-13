# Intmax2 

## Usage

The general syntax for using the CLI tool is:

```
intmax2-cli <COMMAND> [OPTIONS]
```

### Available Commands

1. **Transaction (tx)**
   
   Send a transaction to another address.

   ```
   intmax2-cli tx --block_builder_url <BLOCK_BUILDER_URL>  --private-key <PRIVATE_KEY> --to <RECIPIENT_ADDRESS> --amount <AMOUNT> --token-index <TOKEN_INDEX>
   ```

2. **Deposit**
   
   Deposit funds into the Intmax2 system.

   ```
   intmax2-cli deposit --rpc-url <RPC_URL> --eth-private-key <ETH_PRIVATE_KEY> --private-key <PRIVATE_KEY> --amount <AMOUNT> --token-index <TOKEN_INDEX>
   ```

3. **Sync**
   
   Synchronize your account with the latest state.

   ```
   intmax2-cli sync --private-key <PRIVATE_KEY>
   ```

4. **Balance**
   
   Check the balance of your account.

   ```
   intmax2-cli balance --private-key <PRIVATE_KEY>
   ```

### Options

- `--block_builder_url`: URL of the block builder
- `--private-key`: Your private key (in H256 format)
- `--to`: Recipient's address (in H256 format)
- `--amount`: Amount to send or deposit (in U256 format)
- `--token-index`: Index of the token (u32)
- `--rpc-url`: URL of the Ethereum RPC node
- `--eth-private-key`: Ethereum private key for deposits (in H256 format)

## Examples

1. Make a deposit:
   ```
   cargo run -r --  deposit --rpc-url "" --eth-private-key 0x186aab4d91978e03f84890147e0e4bc114c8188588deb2c58bd877f5911ad78c --private-key 0x186aab4d91978e03f84890147e0e4bc114c8188588deb2c58bd877f5911ad78c --amount 100000000 --token-index 0
   ```
2. Check your balance:
   ```
   cargo run -r -- balance --private-key 0x186aab4d91978e03f84890147e0e4bc114c8188588deb2c58bd877f5911ad78c
   ```
3. Send a transaction:
   ```
   cargo run -r -- tx --block_builder_url "" --private-key 0x186aab4d91978e03f84890147e0e4bc114c8188588deb2c58bd877f5911ad78c --to 0x186aab4d91978e03f84890147e0e4bc114c8188588deb2c58bd877f5911ad78c --amount 100000000 --token-index 0
   ```


3. Sync your account:
   ```
   intmax2-cli sync --private-key 0x...
   ```

4. Check your balance:
   ```
   intmax2-cli balance --private-key 0x...
   ```
