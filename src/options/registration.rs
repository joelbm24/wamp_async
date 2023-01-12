use crate::{
    WampDict,
    Arg
};
pub use crate::options::option::{
    OptionBuilder,
    WampOption,
    MatchOption,
    InvokeOption,
};

/// Base struct for storing WampDict value
pub struct RegistrationOptionItem(Option<WampDict>);

/// Provides functions for adding defined options to the WampDict
impl RegistrationOptionItem {
    /// Add an option for pattern matching the topic of the subscription
    pub fn with_invoke(&self, invoke_option: InvokeOption) -> Self {
        self.with_option(WampOption::RegisterOption("invoke".to_owned(), Arg::String(invoke_option.value())))
    }

    pub fn with_match(&self, match_option: MatchOption) -> Self {
        self.with_option(WampOption::RegisterOption("match".to_owned(), Arg::String(match_option.value())))
    }
}

/// Add base OptionBuilder functionality
impl OptionBuilder for RegistrationOptionItem {
    /// Build a new SubscriptionOptionItem from a provided Option<WampDict>
    fn create(options: Option<WampDict>) -> Self where Self: OptionBuilder + Sized {
        Self(options)
    }

    /// Return the WampDict being operated on and stored by SubscriptionOptionItem
    fn get_dict(&self) -> Option<WampDict> {
        self.0.clone()
    }
}

/// Default
impl Default for RegistrationOptionItem {
    /// Create a new empty SubscriptionOptionItem
    fn default() -> Self {
        Self::empty()
    }
}

/// Alias for SubscriptionOptionItem
pub type RegistrationOptions = RegistrationOptionItem;