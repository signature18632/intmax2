import init, { add } from '../pkg';

async function main() {


  try {
    const result = await add(5, 3);
    console.log('Result:', result); 
  } catch (error) {
    console.error('Error:', error);
  }
}

main();