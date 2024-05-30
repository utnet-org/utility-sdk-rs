use unc_sdk::env;
use unc_sdk::serde::Serialize;
use unc_sdk::serde_json;

#[derive(Serialize, Debug)]
#[serde(crate = "unc_sdk::serde")]
#[serde(tag = "standard")]
#[must_use = "don't forget to `.emit()` this event"]
#[serde(rename_all = "snake_case")]
pub(crate) enum UncEvent<'a> {
    Uip171(crate::non_fungible_token::events::Uip171Event<'a>),
    Uip141(crate::fungible_token::events::Uip141Event<'a>),
}

impl<'a> UncEvent<'a> {
    fn to_json_string(&self) -> String {
        // Events cannot fail to serialize so fine to panic on error
        #[allow(clippy::redundant_closure)]
        serde_json::to_string(self).ok().unwrap_or_else(|| env::abort())
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub(crate) fn emit(self) {
        unc_sdk::env::log_str(&self.to_json_event_string());
    }
}
