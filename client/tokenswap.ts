import {
  Keypair,
  Connection,
  PublicKey,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  createMint,
  createTransferInstruction,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  transfer,
} from "@solana/spl-token";
import fs from "mz/fs";
import path from "path";

import { getPayer, getRpcUrl, createKeypairFromFile } from "./utils";

let connection: Connection;
let payer: Keypair;
let programId: PublicKey;
const PROGRAM_PATH = path.resolve(__dirname, "../dist/program");
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, "tokenswap.so");
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, "tokenswap-keypair.json");

export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, "confirmed");
  const version = await connection.getVersion();
  console.log("Connection to cluster established:", rpcUrl, version);
}

export async function establishPayer(): Promise<void> {
  if (!payer) {
    payer = await getPayer();
  }
  /* Request airdrop if need */
  // const sig = await connection.requestAirdrop(
  //   payer.publicKey,
  //   1 * LAMPORTS_PER_SOL
  // );
  // await connection.confirmTransaction(sig);
  console.log("Using account", payer.publicKey.toBase58());
}

export async function checkProgramHashBeenDeployed(): Promise<void> {
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/tokenswap.so\``
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        "Program needs to be deployed with `solana program deploy dist/program/tokenswap.so`"
      );
    } else {
      throw new Error("Program needs to be built and deployed");
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);
}
