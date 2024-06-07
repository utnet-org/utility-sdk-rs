use unc_sdk::{ext_contract, json_types::U128, AccountId};

/// [`FungibleTokenResolver`] provides token transfer resolve functionality.
///
/// # Examples
///
/// ```
/// use unc_sdk::{unc, PanicOnDefault, AccountId, log};
/// use unc_sdk::collections::LazyOption;
/// use unc_sdk::json_types::U128;
/// use unc_contract_standards::fungible_token::{FungibleToken, FungibleTokenResolver};
/// use unc_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
///
/// #[unc(contract_state)]
/// #[derive(PanicOnDefault)]
/// pub struct Contract {
///     token: FungibleToken,
///     metadata: LazyOption<FungibleTokenMetadata>,
/// }
///
///#[unc]
/// impl FungibleTokenResolver for Contract {
///     #[private]
///     fn ft_resolve_transfer(
///         &mut self,
///         sender_id: AccountId,
///         receiver_id: AccountId,
///         amount: U128,
///     ) -> U128 {
///         let (used_amount, burned_amount) =
///             self.token.internal_ft_resolve_transfer(&sender_id, receiver_id, amount);
///         if burned_amount > 0 {
///             log!("Account @{} burned {}", sender_id, burned_amount);
///         }
///         used_amount.into()
///     }
/// }
/// ```
///
#[ext_contract(ext_ft_resolver)]
pub trait FungibleTokenResolver {
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128;
}
