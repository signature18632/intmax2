import { add } from '../pkg';

async function main() {
  try {
    const result = await add(3, 2);
    console.log(result);
  } catch (err) {
    console.log(err);
    console.log('Error calling add');
  }
}

main().then(() => {
  process.exit(0);
}).catch((err) => {
  console.error(err);
  process.exit(1);
});