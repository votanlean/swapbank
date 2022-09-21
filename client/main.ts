import {
  checkProgramHashBeenDeployed,
  establishConnection,
  establishMint,
  establishPayer,
  establishPayerAta,
  establishVault,
  establishVaultAta,
  initialize,
  mintToPayerAta,
  mintToVaultAta,
  swapSolToToken,
  swapTokenToSol,
} from "./tokenswap";

async function main() {
  await establishConnection();
  await establishPayer();
  await establishMint();
  await establishPayerAta();
  await mintToPayerAta();
  await checkProgramHashBeenDeployed();
  await establishVault();
  await establishVaultAta();
  await mintToVaultAta();
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
