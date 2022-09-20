import {
  checkProgramHashBeenDeployed,
  establishConnection,
  establishMint,
  establishPayer,
  establishVault,
  initialize,
  swapSolToToken,
  swapTokenToSol,
} from "./swapbank";

async function main() {
  await establishConnection();
  await establishPayer();
  await establishMint();
  await checkProgramHashBeenDeployed();
  await establishVault();
  await initialize();
  switch (process.argv.slice(2)[0]) {
    case "1":
      await swapSolToToken();
      break;
    case "2":
      await swapTokenToSol();
      break;
    default:
      throw console.error("Invalid instruction");
  }
  console.log("Success");
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
