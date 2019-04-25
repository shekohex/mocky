use std::{env, fs, io, path::PathBuf};

use itertools::Itertools;
use smush::{Encoding, Quality};

fn main() -> io::Result<()> {
  let features = [
    "address", "company", "date", "lorem", "name", "phone", "system",
  ];
  let locales = ["en", "fr"];
  let mut paths = Vec::with_capacity(features.len());
  for (feat, locale) in features.iter().cartesian_product(&locales) {
    paths.push(PathBuf::from(format!(
      "./src/locales/{}/{}.json",
      locale, feat
    )));
  }
  for path in paths {
    if path.exists() {
      let data = fs::read(&path)?;
      let bin = smush::encode(&data, Encoding::Brotli, Quality::Maximum)?;
      let p = path.with_extension("br");
      fs::write(p, bin)?;
    }
  }
  Ok(())
}
