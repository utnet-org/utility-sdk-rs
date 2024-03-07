use unc_sdk::UncSchema;

struct Inner;

#[derive(UncSchema)]
struct Outer(Inner);

#[derive(UncSchema)]
#[abi]
struct Nada;

#[derive(UncSchema)]
#[abi()]
struct Empty;

#[derive(UncSchema)]
#[abi(serde)]
struct SingleUnexpected;

#[derive(UncSchema)]
#[abi(json, serde)]
struct OneUnexpected;

#[derive(UncSchema)]
#[abi(json, serde, schemars)]
struct TwoUnexpected;

#[derive(UncSchema)]
#[abi(json, serde = "?")]
struct OneUnexpectedPath;

#[derive(UncSchema)]
union Unsupporteed {
    a: u8,
    b: u16,
}

#[derive(UncSchema)]
#[abi()]
union UnsupporteedWithoutArgs {
    a: u8,
    b: u16,
}

#[derive(UncSchema)]
#[abi(json)]
union UnsupporteedWithArgs {
    a: u8,
    b: u16,
}

fn main() {}
