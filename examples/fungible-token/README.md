Fungible Token (FT)
===================

Example implementation of a [Fungible Token] contract which uses [unc-contract-standards].

  [Fungible Token]: https://xxx.io/Standards/Tokens/FungibleTokenCore.html
  [unc-contract-standards]: https://github.com/utnet-org/utility-sdk-rs/tree/master/unc-contract-standards

NOTES:

- The maximum balance value is limited by U128 (2**128 - 1).
- JSON calls should pass U128 as a base-10 string. E.g. "100".
- This does not include escrow functionality, as `ft_transfer_call` provides a superior approach. An escrow system can, of course, be added as a separate contract.

## Building

To build run:

```bash
./build.sh
```

## Testing

To test run:

```bash
cargo test --package fungible-token -- --nocapture
```

## Deploy

```sh
unc deploy --wasmFile res/fungible_token.wasm --accountId $CONTRACT_ID
```

FT contract should be initialized before usage.
The next command will initialize the contract using the `new` method:

```bash
unc call $CONTRACT_ID new '{"owner_id": "'$CONTRACT_ID'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Example Token Name", "symbol": "EXLT", "decimals": 8 }}' --accountId $CONTRACT_ID
```

To get the fungible token metadata:

```bash
unc view $CONTRACT_NAME ft_metadata
```

## Transfer

Let's set up an account to transfer some tokens to. These account will be a sub-account of the Utility account you logged in with.

```sh
unc create-account bob --initialBalance 1
```

Add storage deposit for Bob's account:

```sh
unc call $CONTRACT_ID storage_deposit '' --accountId bob --amount 0.00125
```

Check balance of Bob's account, it should be `0` for now:

```sh
unc view $CONTRACT_ID ft_balance_of '{"account_id": "'bob'"}'
```

Transfer tokens to Bob from the contract that minted these fungible tokens, exactly 1 yoctoUtility of deposit should be attached:

```sh
unc call $CONTRACT_ID ft_transfer '{"receiver_id": "'bob'", "amount": "19"}' --accountId $CONTRACT_ID --amount 0.000000000000000000000001
```

Check the balance of Bob again with the command from before and it will now return `19`.

## Changelog

### `0.2.0`

- Switched form using [UIP-1](https://github.com/utnet-org/UIPs/pull/1) to [UIP-1](https://github.com/utnet-org/UIPs/issues/141).

### `0.1.0`

#### Breaking storage change

- Switching `UnorderedMap` to `LookupMap`. It makes it cheaper and faster due to decreased storage access.
