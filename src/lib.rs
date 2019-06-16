#![warn(missing_docs)]
#![deny(missing_debug_implementations, unsafe_code, nonstandard_style)]
#![warn(rust_2018_idioms)]

//! Mocky
use crate::address::Address;

mod address;
mod utils;

#[derive(Debug)]
pub(crate) enum MockyLocale {
  #[cfg(feature = "localization-en")]
  En,
  #[cfg(feature = "localization-fr")]
  Fr,
}

#[cfg(feature = "localization-en")]
impl Default for MockyLocale {
  fn default() -> Self { MockyLocale::En }
}

#[cfg(all(feature = "localization-fr", not(feature = "localization-en")))]
impl Default for MockyLocale {
  fn default() -> Self { MockyLocale::Fr }
}

#[derive(Debug, Default)]
struct Mocky {
  locale: MockyLocale,
}

impl Mocky {
  pub fn new(locale: MockyLocale) -> Self { Self { locale } }

  #[cfg(feature = "address")]
  pub fn address(&self) -> &Address { Address::new(&self.locale) }
}
