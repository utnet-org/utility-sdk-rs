# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.1.0](https://github.com/utnet-org/utility-sdk-rs/compare/unc-contract-standards-v2.0.2...unc-contract-standards-v2.1.0) - 2024-05-28

### Added
- Finalize `#[unc]` attribute-macro implementation with the support for custom parameters passing to serializer attributes `#[unc(serializers = [borsh(...)])]` ([#11](https://github.com/utnet-org/utility-sdk-rs/pull/11))
- Introduce `#[unc]` macro to further streamline contracts development reducing the boilerplate! ([#12](https://github.com/utnet-org/utility-sdk-rs/pull/12))
### Other
- Update deprecated items to use the correct version number
