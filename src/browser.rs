use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::fmt::Display;

use crate::config::{APPLE_WEB_KIT, CHROME, EDGE, FIREFOX, OPERA, SAFARI};

// from https://useragentstring.com/pages/Firefox/
pub enum Browser {
    Chrome,
    Opera,
    Firefox,
    Safari,
    // IEnternetExplorer,
    Edge,
}

impl Browser {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0..120) {
            0..=29 => Browser::Chrome,
            30..=39 => Browser::Opera,
            40..=69 => Browser::Firefox,
            70..=89 => Browser::Safari,
            90..=120 => Browser::Edge,
            _ => Browser::Edge,
        }
    }
}

impl Distribution<Browser> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Browser {
        Browser::random(rng)
    }
}

impl Display for Browser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rng = rand::thread_rng();
        match self {
            Browser::Chrome => write!(
                f,
                "{} Chrome/{}",
                APPLE_WEB_KIT[0],
                CHROME[rng.gen_range(0..CHROME.len())],
            ),
            Browser::Opera => write!(f, "{}", OPERA[rng.gen_range(0..OPERA.len())]),
            Browser::Firefox => {
                write!(f, "{}", FIREFOX[rng.gen_range(0..FIREFOX.len())])
            }
            Browser::Safari => write!(
                f,
                "{} {}",
                APPLE_WEB_KIT[0],
                SAFARI[rng.gen_range(0..SAFARI.len())]
            ),
            // Browser::IEnternetExplorer => todo!(),
            Browser::Edge => write!(
                f,
                "{} {}",
                APPLE_WEB_KIT[0],
                EDGE[rng.gen_range(0..EDGE.len())]
            ),
        }
    }
}
