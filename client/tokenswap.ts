import {
  Keypair,
  Connection,
  PublicKey,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
  SystemProgram,
} from "@solana/web3.js";
import {
  Account,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import fs from "mz/fs";
import path from "path";

import { getPayer, getRpcUrl, createKeypairFromFile } from "./utils";
import { BN } from "bn.js";

let connection: Connection;
let mint: PublicKey;
let payer: Keypair;
let payerAta: Account;
let programId: PublicKey;
let vault: PublicKey;
let vaultAta: Account;
const DECIMALS = 9;
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
  console.log("Using payer", payer.publicKey.toBase58());
}

export async function establishMint(): Promise<void> {
  mint = await createMint(
    connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    DECIMALS
  );
  console.log(`Using mint ${mint.toBase58()}`);
}

export async function establishPayerAta(): Promise<void> {
  payerAta = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    payer.publicKey
  );
  console.log(`Using payerAta ${payerAta.address.toBase58()}`);
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

export async function establishVault(): Promise<void> {
  const [vaultAddress] = await PublicKey.findProgramAddress(
    [Buffer.from("vault"), mint.toBuffer()],
    programId
  );
  vault = vaultAddress;
  console.log(`Using vault ${vault.toBase58()}`);
}

export async function establishVaultAta(): Promise<void> {
  vaultAta = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    vault,
    true
  );
  console.log(`Using vaultAta ${vaultAta.address.toBase58()}`);
}

export async function mintToVaultAta(): Promise<void> {
  await mintTo(
    connection,
    payer,
    mint,
    vaultAta.address,
    payer,
    1000 * LAMPORTS_PER_SOL
  );
  console.log(`Mint 1000 tokens to vaultAta ${vaultAta.address.toBase58()}`);
}

export async function initialize(): Promise<void> {
  const instructionData = Buffer.from(Uint8Array.of(0));
  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: vault,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: instructionData,
  });

  const txSig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer]
  );
  console.log(
    `Finish initialize, more info: \nhttps://explorer.solana.com/tx/${txSig}?cluster=custom`
  );
}

export async function swapSolToToken(): Promise<void> {
  const instructionData = Buffer.from(
    Uint8Array.of(1, ...new BN(0.1 * LAMPORTS_PER_SOL).toArray("le", 8))
  );

  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: payerAta.address,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: programId,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vault,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vaultAta.address,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
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
    `Finish swap Sol to token, more info:  \nhttps://explorer.solana.com/tx/${swapSig}?cluster=custom`
  );
}

export async function swapTokenToSol(): Promise<void> {
  const instructionData = Buffer.from(
    Uint8Array.of(2, ...new BN(1000).toArray("le", 8))
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
        pubkey: swapBankAta,
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
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: programId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: programAta.address,
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
    `Finish swap Token to SOL, more info: \nhttps://explorer.solana.com/tx/${swapSig}?cluster=custom`
  );
}
