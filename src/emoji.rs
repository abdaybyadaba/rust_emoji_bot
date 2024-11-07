
use rand::{thread_rng, Rng};

const EMOJIES1: [&str; 5] = [
    "🎄",
    "👍",
    "🤝",
    "👏",
    "🔥",
]; // first set of emojies (Good)

const EMOJIES2: [&str; 5] = [
    "🎄",
    "😈",
    "😨",
    "😐",
    "👍",
]; // second set of emojies (Better)

pub enum EMOJIES {
    Good,
    Better,
}

#[must_use]
fn choose_emoji(link: &[&str]) -> String {
    link[thread_rng().gen_range(0..link.len())].to_owned()
}

impl EMOJIES {
    #[must_use]
    pub fn give_emoji(&self) -> String {
        match *self {
            EMOJIES::Good => choose_emoji(&EMOJIES1),
            EMOJIES::Better => choose_emoji(&EMOJIES2),
        }
    }
}