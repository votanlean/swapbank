import {
  checkProgramHashBeenDeployed,
  establishConnection,
  establishPayer,
  swapToken,
} from "./swapbank";

async function main() {
  await establishConnection();
  await establishPayer();
  await checkProgramHashBeenDeployed();
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
