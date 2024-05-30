use unc_sdk::unc_bindgen;

#[unc_bindgen(contract_metadata(
    version = "39f2d2646f2f60e18ab53337501370dc02a5661c",
    link = "https://github.com/utnet-org-examples/nft-tutorial",
    standard(standard = "uip330", version = "1.1.0"),
    standard(standard = "uip171", version = "1.0.0"),
    standard(standard = "uip177", version = "2.0.0"),
))]
struct Contract {}

#[unc_bindgen]
impl Contract {}

fn main() {}
