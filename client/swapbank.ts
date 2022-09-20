import {
  Keypair,
  Connection,
  PublicKey,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  Account,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  createMint,
  createTransferInstruction,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  transfer,
} from "@solana/spl-token";
import fs from "mz/fs";
import path from "path";

import { getPayer, getRpcUrl, createKeypairFromFile } from "./utils";
import { BN } from "bn.js";

let connection: Connection;
let payer: Keypair;
let payerAta: Account;
let programId: PublicKey;
let mintA: PublicKey;
let mintB: PublicKey;
let vaultA: PublicKey;
let vaultB: PublicKey;
let vaultAAta: Account;
let vaultBAta: Account;
let swapBank: PublicKey;
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

export async function initialize(): Promise<void> {
  mintA = await createMint(
    connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    9
  );

  payerAta = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mintA,
    payer.publicKey
  );
  await mintTo(
    connection,
    payer,
    mintA,
    payerAta.address,
    payer,
    1e9 * LAMPORTS_PER_SOL
  );
  console.log(
    `mint ${1e9} SOL to payer Token Account Address ${payerAta.address.toBase58()}`
  );

  mintB = await createMint(
    connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    9
  );

  const [swapBankPda, bump] = await PublicKey.findProgramAddress(
    [Buffer.from("swap_bank"), mintA.toBuffer(), mintB.toBuffer()],
    programId
  );
  swapBank = swapBankPda;
  console.log(`pda: ${swapBank.toBase58()}, bump: ${bump}`);

  const [vaultAPda, vaultAPdaBump] = await PublicKey.findProgramAddress(
    [
      Buffer.from("swap_bank"),
      payer.publicKey.toBuffer(),
      mintA.toBuffer(),
      swapBank.toBuffer(),
    ],
    programId
  );
  vaultA = vaultAPda;
  console.log(
    `vaultAPda pda: ${vaultAPda.toBase58()}, vaultAPda bump: ${vaultAPdaBump}`
  );

  vaultAAta = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mintA,
    vaultA,
    true
  );
  await mintTo(
    connection,
    payer,
    mintA,
    vaultAAta.address,
    payer,
    2e9 * LAMPORTS_PER_SOL
  );
  console.log(
    `mint ${2e9} SOL to vautA Token Account Address ${vaultAAta.address.toBase58()}`
  );

  const [vaultBPda, vaultBPdaBump] = await PublicKey.findProgramAddress(
    [
      Buffer.from("swap_bank"),
      payer.publicKey.toBuffer(),
      mintB.toBuffer(),
      swapBank.toBuffer(),
    ],
    programId
  );
  vaultB = vaultBPda;
  console.log(
    `vaultBPda pda: ${vaultBPda.toBase58()}, vaultBPda bump: ${vaultBPdaBump}`
  );

  const instructionData = Buffer.from(Uint8Array.of(0));
  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: swapBank,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mintA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mintB,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vaultA,
        isSigner: false,
        isWritable: true, //double check
      },
      {
        pubkey: vaultB,
        isSigner: false,
        isWritable: true, //double check
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: instructionData,
  });

  const swapSig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer]
  );
  console.log(
    `visit \nhttps://explorer.solana.com/tx/${swapSig}?cluster=custom`
  );
}

export async function swapToken(): Promise<void> {
  const instructionData = Buffer.from(
    Uint8Array.of(1, ...new BN(1000).toArray("le", 8))
  );

  console.log("vaultA ATA", vaultAAta.address.toBase58());

  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: swapBank,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mintA,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mintB,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vaultA,
        isSigner: false,
        isWritable: true, //double check
      },
      {
        pubkey: vaultB,
        isSigner: false,
        isWritable: true, //double check
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: payerAta.address,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vaultAAta.address,
        isSigner: false,
        isWritable: true,
      },
    ],
    programId,
    data: instructionData,
  });

  const swapSig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer]
  );
  console.log(
    `visit \nhttps://explorer.solana.com/tx/${swapSig}?cluster=custom`
  );
}
