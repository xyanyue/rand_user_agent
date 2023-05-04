use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::fmt::Display;
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

const APPLE_WEB_KIT: [&str; 2] = [
    "AppleWebKit/537.36 (KHTML, like Gecko)",
    "AppleWebKit/605.1.15 (KHTML, like Gecko)",
];
const EDGE: [&str; 10] = [
    "Chrome/70.0.3538.102 Safari/537.36 Edge/18.19582",
    "Chrome/70.0.3538.102 Safari/537.36 Edge/18.19577",
    "Chrome/64.0.3282.140 Safari/537.36 Edge/18.17720",
    "Chrome/86.0.8810.3391 Safari/537.36 Edge/18.14383",
    "Edge/17.10859 Safari/452.6",
    "Chrome/51.0.2704.79 Safari/537.36 Edge/14.14931",
    "Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393",
    "Chrome/46.0.2486.0 Safari/537.36 Edge/13.9200",
    "Chrome/46.0.2486.0 Safari/537.36 Edge/13.10586",
    "Chrome/42.0.2311.135 Safari/537.36 Edge/12.246",
];
const OPERA: [&str; 2] = [
    "Gecko/20100101 Firefox/4.0 Opera 12.14",
    "Firefox/14.0 Opera/12.0",
];
impl Display for Browser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rng = rand::thread_rng();
        match self {
            Browser::Chrome => write!(
                f,
                "{} Chrome/{}.0.{}.{}",
                APPLE_WEB_KIT[0],
                rng.gen_range(100..110),
                rng.gen_range(1000..5060),
                rng.gen_range(10..90)
            ),
            Browser::Opera => write!(
                f,
                "{} {}",
                APPLE_WEB_KIT[0],
                OPERA[rng.gen_range(0..OPERA.len())]
            ),
            Browser::Firefox => {
                let v = rng.gen_range(30..41);
                write!(f, "Gecko/{}.0 Firefox/{}.0", v, v)
            }
            Browser::Safari => write!(
                f,
                "{} Safari/{}.{}.{}",
                APPLE_WEB_KIT[0],
                rng.gen_range(400..500),
                rng.gen_range(20..80),
                rng.gen_range(10..90),
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
