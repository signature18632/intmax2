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
   intmax2-cli tx --private-key <PRIVATE_KEY> --to <RECIPIENT_ADDRESS> --amount <AMOUNT> --token-index <TOKEN_INDEX>
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

- `--private-key`: Your private key (in H256 format)
- `--to`: Recipient's address (in H256 format)
- `--amount`: Amount to send or deposit (in U256 format)
- `--token-index`: Index of the token (u32)
- `--rpc-url`: URL of the Ethereum RPC node
- `--eth-private-key`: Ethereum private key for deposits (in H256 format)

## Examples

1. Send a transaction:
   ```
   intmax2-cli tx --private-key 0x... --to 0x... --amount 1000000000000000000 --token-index 0
   ```

2. Make a deposit:
   ```
   intmax2-cli deposit --rpc-url https://mainnet.infura.io/v3/YOUR-PROJECT-ID --eth-private-key 0x... --private-key 0x... --amount 1000000000000000000 --token-index 0
   ```

3. Sync your account:
   ```
   intmax2-cli sync --private-key 0x...
   ```

4. Check your balance:
   ```
   intmax2-cli balance --private-key 0x...
   ```
