use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use serde::Deserialize;
use smush::Encoding;

use crate::{utils, MockyLocale};

#[cfg(feature = "localization-en")]
lazy_static! {
  static ref ADDRESS_EN_BYTES: Vec<u8> =
    smush::decode(include_bytes!("./locales/en/address.br"), Encoding::Brotli)
      .unwrap();
  pub(crate) static ref ADDRESS_EN: Address =
    serde_json::from_slice(&ADDRESS_EN_BYTES).unwrap();
}

#[cfg(feature = "localization-fr")]
lazy_static! {
  static ref ADDRESS_FR_BYTES: Vec<u8> =
    smush::decode(include_bytes!("./locales/fr/address.br"), Encoding::Brotli)
      .unwrap();
  pub(crate) static ref ADDRESS_FR: Address =
    serde_json::from_slice(&ADDRESS_FR_BYTES).unwrap();
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
      FR => &ADDRESS_FR,
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
