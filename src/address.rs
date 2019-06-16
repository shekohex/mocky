use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::{utils, MockyLocale};

#[cfg(feature = "localization-en")]
const ADDRESS_EN_BYTES: &'static [u8] =
  include_bytes!(concat!(env!("OUT_DIR"), "/en/address.cbor"));

#[cfg(feature = "localization-en")]
lazy_static! {
  static ref ADDRESS_EN: Address =
    serde_cbor::from_slice(&ADDRESS_EN_BYTES).unwrap();
}
#[cfg(feature = "localization-fr")]
const ADDRESS_FR_BYTES: &'static [u8] =
  include_bytes!(concat!(env!("OUT_DIR"), "/fr/address.cbor"));

#[cfg(feature = "localization-fr")]
lazy_static! {
  static ref ADDRESS_FR: Address =
    serde_cbor::from_slice(&ADDRESS_FR_BYTES).unwrap();
}

#[cfg(feature = "address")]
#[derive(Deserialize, Debug)]
pub(crate) struct Address {
  #[serde(borrow)]
  states: Vec<&'static str>,
  #[serde(borrow)]
  #[serde(default)]
  states_abbr: Vec<&'static str>,
  #[serde(borrow)]
  #[serde(default)]
  postcode: Vec<&'static str>,
}

impl Address {
  pub fn new(locale: &MockyLocale) -> &'static Self {
    use MockyLocale::*;
    match locale {
      #[cfg(feature = "localization-en")]
      En => &ADDRESS_EN,
      #[cfg(feature = "localization-fr")]
      Fr => &ADDRESS_FR,
    }
  }

  pub fn state(&self) -> &str {
    self
      .states
      .choose(&mut rand::thread_rng())
      .unwrap_or_else(|| &"")
  }

  pub fn state_abbr(&self) -> &str {
    self
      .states_abbr
      .choose(&mut rand::thread_rng())
      .unwrap_or_else(|| &"")
  }

  /// Generates random zip code from format. If format is not specified, the
  /// locale's zip format is used.
  pub fn zip_code(&self) -> String {
    let postcode = self
      .states_abbr
      .choose(&mut rand::thread_rng())
      .unwrap_or_else(|| &"#####-####");
    utils::replace_symbols(postcode)
  }
}

#[cfg(feature = "localization-en")]
#[test]
fn can_deserialize_en() {
  // Ensure accessing lazy address doesn't panic
  let address: &Address = &*ADDRESS_EN;
}

#[cfg(feature = "localization-fr")]
#[test]
fn can_deserialize_fr() {
  // Ensure accessing lazy address doesn't panic
  let address: &Address = &*ADDRESS_FR;
}
