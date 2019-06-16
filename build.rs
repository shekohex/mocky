use std::{env, fs, io::prelude::*, path::PathBuf};

use itertools::Itertools;

fn main() {
  let features: &[&str] = &[
    #[cfg(feature = "address")]
    "address",
    #[cfg(feature = "company")]
    "company",
    #[cfg(feature = "date")]
    "date",
    #[cfg(feature = "lorem")]
    "lorem",
    #[cfg(feature = "name")]
    "name",
    #[cfg(feature = "phone")]
    "phone",
    #[cfg(feature = "system")]
    "system",
  ];
  let locales: &[&str] = &[
    #[cfg(feature = "localization-en")]
    "en",
    #[cfg(feature = "localization-fr")]
    "fr",
  ];

  // If either array is empty, but the other is not
  if locales.is_empty() ^ features.is_empty() {
    let mut empty_type = "language";
    let mut non_empty_type = "data type";
    if features.is_empty() {
      std::mem::swap(&mut empty_type, &mut non_empty_type);
    }
    println!("cargo:warning={} feature(s) were enabled, but no {} features were enabled. No mock data will created.",
      non_empty_type,
      empty_type,
    );
  }

  let output_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
  for locale in locales {
    fs::create_dir_all(output_path.join(locale)).unwrap();
  }
  let paths = features.iter().cartesian_product(locales);
  for (feat, locale) in paths {
    let path = PathBuf::from(format!("./src/locales/{}/{}.json", locale, feat));
    println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    if !path.exists() {
      continue;
    }
    let mut out_path = output_path.join(locale);
    out_path.push(path.file_name().unwrap());
    out_path.set_extension("cbor");

    let input = fs::File::open(path).unwrap();

    let mut output = fs::File::create(out_path).unwrap();
    let mut deserializer = serde_json::Deserializer::from_reader(input);
    let mut serializer = serde_cbor::Serializer::new(&mut output);

    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
    drop(serializer);
    output.flush().unwrap();
  }
}
