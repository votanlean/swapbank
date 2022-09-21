# TokenSwap

Can Swap SOL to token

Use Vault as program PDA to transfer SOL
Use VaultAta as an Associated Token Account to transfer Token

## Local Installation

```bash
solana-test-validator --reset
yarn install
yarn build
yarn deploy
```

## Usage

Test swap 1 SOL to 0.1 Token

```bash
yarn start 1
```

Test swap 1 Token to 0.1 SOL

```bash
yarn start 2
```

## DevNet test

Program address: https://explorer.solana.com/address/6LWHXwRipjjuBdkuVJTJqd3y9NYXEuhpi5FnQGYFn7b8?cluster=devnet

### Swap 0.1 SOL to 1 Token

```bash
yarn start 1
```

### Result:

https://explorer.solana.com/tx/3yv8GbUeQjKFNrrCkXYw3rb83DG44M7TWMeDuhTCrATNTdciG7oNdJwLjWcDrHSKuvW4Gr5jz1sNU2u9SkuK6Hmo?cluster=devnet

![image](https://user-images.githubusercontent.com/12319377/191399985-ac0499b6-929d-4412-9507-9afdbb5f4c7c.png)


```
Connection to cluster established: https://api.devnet.solana.com { 'feature-set': 872682205, 'solana-core': '1.10.39' }
Current balance is 5.17908108
Using payer AQZUEnSWcsyA5otxntdfZbQ34Nus3SrDeBCW57JxREMH
Using mint 8N1RRi3LLE5PGQwHuc2Qg6SJhwNXRjPpNCrgUyTTXx6U
Using payerAta 3H26nKgq3G81yDm74nhkrmr1qE52eifS6j3zf9PPiNSr
Mint 1000 tokens to payerAta 3H26nKgq3G81yDm74nhkrmr1qE52eifS6j3zf9PPiNSr
Using program 6LWHXwRipjjuBdkuVJTJqd3y9NYXEuhpi5FnQGYFn7b8
Using vault 7aP1wSyVwPvDTm8qHFAwXNWgnowVQyS4Gu9YFJjqWcvg
Using vaultAta 37rhJ6KJJkxQphT5iYiaVeUMEd8orHAcG3UhY9GvyrZK
Mint 1000 tokens to vaultAta 37rhJ6KJJkxQphT5iYiaVeUMEd8orHAcG3UhY9GvyrZK
Finish initialize, more info:
https://explorer.solana.com/tx/8Dj7NpAgxMaMac8JUDuFN2nF6ucGG91s1hvQeTnnxT3qp5FW9Y9rnsZjHck1ZXGggjtgxJM4QNuruYCxM8Xf4ub?cluster=custom
Current balance is 0.00155904
Airdropping 1 SOL...
Current balance after airdrop:  1.00155904
Finish swap Sol to Token, more info:
https://explorer.solana.com/tx/3yv8GbUeQjKFNrrCkXYw3rb83DG44M7TWMeDuhTCrATNTdciG7oNdJwLjWcDrHSKuvW4Gr5jz1sNU2u9SkuK6Hmo?cluster=custom
Success
```

### Swap 1 Token to 0.1 SOL

```bash
yarn start 2
```

### Result:

https://explorer.solana.com/tx/qN1iTiVP2h6JLQprLtSMrpjrVQkjEyT8Az6dMk3NP7rBRcLmQ25YdUsmgUa1rdmpBJj3stXVDhE3v5gg8KKkMDg?cluster=devnet

![image](https://user-images.githubusercontent.com/12319377/191400227-f39f5a45-4073-4d9c-9828-1c099a62569f.png)


```
Connection to cluster established: https://api.devnet.solana.com { 'feature-set': 872682205, 'solana-core': '1.10.39' }
Current balance is 5.07194188
Using payer AQZUEnSWcsyA5otxntdfZbQ34Nus3SrDeBCW57JxREMH
Using mint 8wBZ2TTcVp2j5aY1SM9nXVFAk2rSSiQa6abriEKEndjg
Using payerAta CfgjhDggFdjPkwKYL6SL8px7JwopEJqtxzH12HFjrPQ6
Mint 1000 tokens to payerAta CfgjhDggFdjPkwKYL6SL8px7JwopEJqtxzH12HFjrPQ6
Using program 6LWHXwRipjjuBdkuVJTJqd3y9NYXEuhpi5FnQGYFn7b8
Using vault 3o7rDfErSWbT73FD8aNn5nWQRbGtf9RbpfkriwnyHbgN
Using vaultAta FYyvzELfw8mfBdA55NGxCw8pPHs6HCvGybSyns5WwJkz
Mint 1000 tokens to vaultAta FYyvzELfw8mfBdA55NGxCw8pPHs6HCvGybSyns5WwJkz
Finish initialize, more info:
https://explorer.solana.com/tx/3wvhXrMmGVRRWSJyRGzMELfGeSc4pQo1taKV9NuajPUEy69uQRzhJwxFpTyMdambY2gaAKM7mLmZSURoz8DJezLY?cluster=custom
Current balance is 0.00155904
Airdropping 1 SOL...
Current balance after airdrop:  1.00155904
Finish swap Token to SOL, more info:
https://explorer.solana.com/tx/qN1iTiVP2h6JLQprLtSMrpjrVQkjEyT8Az6dMk3NP7rBRcLmQ25YdUsmgUa1rdmpBJj3stXVDhE3v5gg8KKkMDg?cluster=custom
Success
```

## Todo

- [x] Initialize TokenSwap app
- [x] Swap SOL to Token
- [x] Swap Token to Sol
- [x] Javascript scripts to deploy and verify
- [ ] Unittest & Integration test on Rust (Not finish yet, we can use the web3 script to test and verify on the explorer)
