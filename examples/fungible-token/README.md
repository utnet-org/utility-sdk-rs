Fungible Token (FT)
===================

Example implementation of a [Fungible Token] contract which uses [unc-contract-standards].

  [Fungible Token]: https://nomicon.io/Standards/Tokens/FungibleTokenCore.html
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

## Changelog

### `1.0.0`

- Switched form using [UIP-21](https://github.com/utnet-org/UIPs/pull/21) to [UIP-141](https://github.com/utnet-org/UIPs/issues/141).

### `0.3.0`

#### Breaking storage change

- Switching `UnorderedMap` to `LookupMap`. It makes it cheaper and faster due to decreased storage access.

