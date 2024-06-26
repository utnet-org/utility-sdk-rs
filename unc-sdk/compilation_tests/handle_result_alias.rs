use unc_sdk::unc;

type MyResult = Result<u32, &'static str>;

#[derive(Default)]
#[unc(contract_state)]
struct Contract {
    value: u32,
}

#[unc]
impl Contract {
    #[handle_result(aliased)]
    pub fn fun(&self) -> MyResult {
        Err("error")
    }
}

fn main() {}
