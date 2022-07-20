pub use fluent::FluentResource;
use intl_memoizer::concurrent::IntlLangMemoizer;

/// Concurrent version of the FluentBundle
pub type FluentBundle = fluent::bundle::FluentBundle<FluentResource, IntlLangMemoizer>;
