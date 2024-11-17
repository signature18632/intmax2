import { Config, finalize_tx, generate_key_from_provisional, get_user_data, JsGenericAddress, JsTransfer, mimic_deposit, prepare_deposit, query_proposal, send_tx_request, sync, } from '../pkg';
import { constructBlock, postBlock, postEmptyBlock, syncValidityProof } from './state-manager';
import { generateRandom32Bytes } from './utils';

async function main() {
  const baseUrl = "http://localhost:9563";
  const config = Config.new(baseUrl, baseUrl, baseUrl, baseUrl, 3600n, 500n);

  // generate key
  const provisionalPrivateKey = generateRandom32Bytes();
  const key = await generate_key_from_provisional(provisionalPrivateKey);
  const publicKey = key.pubkey;
  const privateKey = key.privkey;
  console.log("privateKey: ", privateKey);
  console.log("publicKey: ", publicKey);

  // deposit to the account
  const tokenIndex = 0; // 0 for ETH
  const amount = "123";
  const pubkeySaltHash = await prepare_deposit(config, privateKey, amount, tokenIndex);
  console.log("pubkeySaltHash: ", pubkeySaltHash);
  await mimic_deposit(baseUrl, pubkeySaltHash, amount);

  // !The following two functions are not used in production.
  await postEmptyBlock(baseUrl); // block builder post empty block
  await syncValidityProof(baseUrl); // block validity prover sync validity proof
  console.log("validity proof synced");

  await new Promise((resolve) => setTimeout(resolve, 5000));

  // sync the account's balance proof 
  await sync(config, privateKey);

  console.log("Sync successful");

  await new Promise((resolve) => setTimeout(resolve, 5000));

  // get the account's balance
  let userData = await get_user_data(config, privateKey);
  let balances = userData.balances;
  for (let i = 0; i < balances.length; i++) {
    const balance = balances[i];
    console.log(`Token ${balance.token_index}: ${balance.amount}`);
  }

  // send a tx 
  const genericAddress = new JsGenericAddress(true, publicKey);
  const salt = generateRandom32Bytes();
  const transfer = new JsTransfer(genericAddress, 0, "1", salt);
  const transfers = new Array<JsTransfer>();
  transfers.push(transfer);
  const memo = await send_tx_request(config, baseUrl, privateKey, transfers);
  const tx = memo.tx();
  console.log("tx.nonce", tx.nonce);
  console.log("tx.transfer_tree_root", tx.transfer_tree_root);

  //! The following function is not used in production.
  await constructBlock(baseUrl); // block builder construct block

  const proposal = await query_proposal(config, baseUrl, privateKey, tx);
  if (proposal === undefined) {
    throw new Error("No proposal found");
  }
  await finalize_tx(config, baseUrl, privateKey, memo, proposal);

  // !The following function is not used in production.
  await postBlock(baseUrl); // block builder post block
  console.log("Tx successful");

  await new Promise((resolve) => setTimeout(resolve, 5000));

  // sync the account's balance proof
  await sync(config, privateKey);
  console.log("Sync successful");

  userData = await get_user_data(config, privateKey);
  balances = userData.balances;
  for (let i = 0; i < balances.length; i++) {
    const balance = balances[i];
    console.log(`Token ${balance.token_index}: ${balance.amount}`);
  }
}

main().then(() => {
  process.exit(0);
}).catch((err) => {
  console.error(err);
  process.exit(1);
});