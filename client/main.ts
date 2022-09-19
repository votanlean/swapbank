import {
  checkProgramHashBeenDeployed,
  establishConnection,
  establishPayer,
} from "./tokenswap";

async function main() {
  await establishConnection();
  await establishPayer();
  await checkProgramHashBeenDeployed();
  console.log("Success");
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
