import {
  checkProgramHashBeenDeployed,
  establishConnection,
  establishPayer,
  initialize,
  swapToken,
} from "./swapbank";

async function main() {
  await establishConnection();
  await establishPayer();
  await checkProgramHashBeenDeployed();
  await initialize();
  await swapToken();
  console.log("Success");
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
