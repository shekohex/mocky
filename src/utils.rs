use rand::{
  distributions::{Distribution, Uniform},
  seq::SliceRandom,
  Rng,
};

/// parses string for symbols (numbers or letters) and replaces them
/// appropriately (# will be replaced with number, ? with letter and * will be
/// replaced with number or letter)
pub(crate) fn replace_symbols(pattern: &str) -> String {
  let mut rng = rand::thread_rng();
  let mut result = String::with_capacity(pattern.len());
  let mut alpha = Uniform::new_inclusive(b'A', b'Z');
  let mut decimal = Uniform::new_inclusive(b'0', b'9');
  for c in pattern.chars() {
    let out_char = match c {
      '#' => char::from(decimal.sample(&mut rng)),
      '?' => char::from(alpha.sample(&mut rng)),
      '*' => {
        let chance: bool = rng.gen();
        if chance {
          char::from(decimal.sample(&mut rng))
        } else {
          char::from(alpha.sample(&mut rng))
        }
      },
      _ => c,
    };
    result.push(out_char);
  }

  result
}

#[cfg(test)]
mod tests_utils {
  use super::*;

  #[test]
  fn test_replace_symbols() {
    let data = "###-###";
    let result = replace_symbols(data);
    println!("{}", result);
    assert_eq!(result.len(), data.len());
    assert_ne!(result, data);
    assert!(result[0..3].chars().all(char::is_numeric));
    assert!(result[4..7].chars().all(char::is_numeric));
    // ----
    let data = "???-???";
    let result = replace_symbols(data);
    println!("{}", result);
    assert_eq!(result.len(), data.len());
    assert_ne!(result, data);
    assert!(result[0..3].chars().all(char::is_alphabetic));
    assert!(result[4..7].chars().all(char::is_alphabetic));
    // ----
    let data = "t*#-#*";
    let result = replace_symbols(data);
    println!("{}", result);
    assert_eq!(result.len(), data.len());
    assert_ne!(result, data);
    assert_eq!(&result[0..1], "t");
    assert!(result[1..2].chars().all(char::is_alphanumeric));
    assert!(result[2..3].chars().all(char::is_numeric));
    assert_eq!(&result[3..4], "-");
    assert!(result[4..5].chars().all(char::is_numeric));
    assert!(result[5..6].chars().all(char::is_alphanumeric));
  }
}
