# Lockable Fungible token

Lockable Fungible token but designed for composability in the async runtime like UNC.

It's an extension of a Fungible Token Standard (UIP#21) with locks.
Locks allow composability of the contracts, but require careful GAS management, because the token contract itself
doesn't guarantee the automatic unlocking call. That's why it shouldn't be used in production
until Safes are implemented from (UIP#26).

## Testing
To test run:

```bash
cargo test --package lockable-fungible-token -- --nocapture
```
