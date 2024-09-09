//! This is a program that randomly generates UserAgents.
//! [这是一个随机生成UserAgent的程序]
//! 数据整理自:
//! [whatmyuseragent.com](https://whatmyuseragent.com/)
//! [user-agents.net](https://user-agents.net/)
//! [useragentstring.com](https://useragentstring.com)
//! # Quick Start
//! ```no run
//!    // one
//!    println!("{}", UserAgent::random().to_string());
//!    // two
//!    UserAgent::custom(rand::random(), rand::random());   
//! ```
//! ## Mobile and computer designated.[手机电脑指定]
//! ```no run
//!    // Random mobile agent. [随机手机端agent]
//!    println!("Mobile:{}", UserAgent::mobile().to_string());
//!    // Random Desktop agent.随机PC端
//!    println!("Pc:{}", UserAgent::pc().to_string());
//! ```
//! ## Fully customizable.完全自定义
//! ```no run
//! let mut rng = rand::thread_rng();
//! println!(
//!     "custom Mobile Iphone Chrome:{}",
//!     UserAgent::custom(Devices::Mobile(MobileDevice::Iphone), Browser::Chrome)
//! );
//! println!(
//!     "custom Desktop Windows random browser :{}",
//!     UserAgent::custom(Devices::Desktop(DesktopDevice::Windows), rand::random())
//! );
//! println!(
//!     "custom random Mobile & random browser :{}",
//!     UserAgent::custom(Devices::Mobile(rand::random()), Browser::random(&mut rng))
//! );
//! //Devices、Browser、DesktopDevice、MobileDevice impl rand::distributions::Standard
//! let mut rng = rand::thread_rng();
//! UserAgent::custom(Devices::random(&mut rng), Browser::random(&mut rng));
//! UserAgent::custom(rand::random(), rand::random());
//! ```
//!
//! # Todo
//! - [ ] create Sec-CH-UA

mod browser;
mod config;
mod os_version;

use std::fmt::Display;

pub use crate::{
    browser::Browser, os_version::DesktopDevice, os_version::Devices, os_version::MobileDevice,
};

/// Agent 惯例..致敬网景
enum Product {
    Mozilla,
    /// 少见先不支持
    Opera,
}
impl<'a> Default for Product {
    fn default() -> Self {
        Product::Mozilla
    }
}

impl<'a> Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Product::Mozilla => write!(f, "Mozilla/5.0"),
            Product::Opera => write!(f, "Opera/9.80"),
        }
    }
}
/// 用户代理生成
/// 分为3段 Product && OS && browser
pub struct UserAgent {
    product: Product,
    os_ver: Devices,
    browser: Browser,
}

impl Display for UserAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.product.to_string(),
            self.os_ver.to_string(),
            self.browser.to_string()
        )
    }
}

impl UserAgent {
    pub fn random() -> Self {
        UserAgent {
            product: Product::Mozilla,
            os_ver: rand::random(),
            browser: rand::random(),
        }
    }

    pub fn mobile() -> Self {
        // let mut rng = rand::thread_rng();
        UserAgent {
            product: Product::Mozilla,
            os_ver: Devices::Mobile(rand::random()),
            browser: rand::random(),
        }
    }
    pub fn pc() -> Self {
        // let mut rng = rand::thread_rng();
        UserAgent {
            product: Product::Mozilla,
            os_ver: Devices::Desktop(rand::random()),
            browser: rand::random(),
        }
    }

    pub fn custom(os_ver: Devices, browser: Browser) -> Self {
        UserAgent {
            product: Product::Mozilla,
            os_ver,
            browser,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        // for _ in 0..100 {
        println!("{}", UserAgent::random().to_string());
        // }
    }

    #[test]
    fn os_check() {
        // for _ in 0..100 {
        println!("Mobile:{}", UserAgent::mobile().to_string());
        println!("Pc:{}", UserAgent::pc().to_string());
        // }
    }

    #[test]
    fn os_custom() {
        let mut rng = rand::thread_rng();
        println!(
            "custom Mobile Iphone Chrome:{}",
            UserAgent::custom(Devices::Mobile(MobileDevice::Iphone), Browser::Chrome)
        );
        println!(
            "custom Desktop Windows random browser :{}",
            UserAgent::custom(Devices::Desktop(DesktopDevice::Windows), rand::random())
        );
        println!(
            "custom random Mobile & random browser :{}",
            UserAgent::custom(Devices::Mobile(rand::random()), Browser::random(&mut rng))
        );
        let mut rng = rand::thread_rng();
        UserAgent::custom(Devices::random(&mut rng), Browser::random(&mut rng));
        UserAgent::custom(rand::random(), Browser::random(&mut rng));
    }
}
