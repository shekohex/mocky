use rand::seq::SliceRandom;

const ALPHA: [char; 26] = [
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
  'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const DECIMALS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// parses string for symbols (numbers or letters) and replaces them
/// appropriately (# will be replaced with number, ? with letter and * will be
/// replaced with number or letter)
pub(crate) fn replace_symbols(pattern: &str) -> String {
  let mut rng = rand::thread_rng();
  let mut result = String::with_capacity(pattern.len());
  for c in pattern.chars() {
    match c {
      '#' => result.push(*DECIMALS.choose(&mut rng).unwrap()),
      '?' => result.push(*ALPHA.choose(&mut rng).unwrap()),
      '*' => {
        let chance: bool = rand::random();
        if chance {
          result.push(*DECIMALS.choose(&mut rng).unwrap())
        } else {
          result.push(*ALPHA.choose(&mut rng).unwrap())
        }
      },
      _ => result.push(c),
    }
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
