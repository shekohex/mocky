use lipsum::*;
use rand::{thread_rng, Rng};

#[cfg(feature = "lorem")]
pub(crate) struct Lorem {}

const MIN_WORDS_PER_PARAGRAPH: usize = 150;
const MAX_WORDS_PER_PARAGRAPH: usize = 200;

impl Lorem {
  pub fn word() -> String { Self::words(1) }

  pub fn words(n: usize) -> String { lipsum_words(n) }

  pub fn paragraph() -> String {
    let mut rng = thread_rng();
    let n: usize =
      rng.gen_range(MIN_WORDS_PER_PARAGRAPH, MAX_WORDS_PER_PARAGRAPH);
    lipsum_words(n)
  }

  pub fn head_paragraph() -> String {
    let mut rng = thread_rng();
    let n: usize =
      rng.gen_range(MIN_WORDS_PER_PARAGRAPH, MAX_WORDS_PER_PARAGRAPH);
    lipsum(n)
  }

  pub fn paragraphs(n: usize) -> Vec<String> {
    let mut paragraphs: Vec<String> = vec![Self::head_paragraph()];

    for _i in 0..n {
      paragraphs.push(Self::paragraph());
    }

    paragraphs
  }

  pub fn title() -> String { lipsum_title() }
}
