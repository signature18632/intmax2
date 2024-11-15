import { Config, generate_key_from_provisional, mimic_deposit, prepare_deposit,  } from '../pkg';
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
  const amount= "123";
  const pubkeySaltHash = await prepare_deposit(config, privateKey, amount, 0);
  await mimic_deposit(baseUrl, publicKey, amount);


  
}

main().then(() => {
  process.exit(0);
}).catch((err) => {
  console.error(err);
  process.exit(1);
});