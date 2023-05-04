//! > 这是一个随机生成UserAgent的程序<初版>。
//!
//! > 未保证浏览器版本真实存在。不生成现实较少见的Agent
//!
//! > 安卓系统版本 使用的中国内地一网站userAgent系统 top n
//!
//! # Quick Start
//! ```no run
//!     for _ in 0..100 {
//!         println!("{}", UserAgent::random().to_string());
//!     }
//! ```
//! ### 手机电脑指定
//! ```no run
//!     for _ in 0..100 {
//!         // 随机手机端agent
//!         println!("Mobile:{}", UserAgent::mobile().to_string());
//!         // 随机PC端
//!         println!("Pc:{}", UserAgent::pc().to_string());
//!     }
//! ```
//! ### 完全自定义
//! ```no run
//!     let mut rng = rand::thread_rng();
//!     for _ in 0..100 {
//!         // 指定 Android & Chrome
//!         println!("{}", UserAgent::custom(OS::Android, Browser::Chrome));
//!         // 指定 Android & 随机浏览器
//!         println!(
//!             "{}",
//!             UserAgent::custom(OS::Android, Browser::random(&mut rng))
//!         );
//!         // 随机手机系统 & 随机浏览器  === UserAgent::mobile
//!         println!(
//!             "{}",
//!             UserAgent::custom(OS::mobile(&mut rng), Browser::random(&mut rng))
//!         );
//!     }
//! ```
//!
//! # Todo
//! - [ ] 生成Sec-CH-UA
//! - [ ] 更多统计信息收集,提取系统版本 和 浏览器真实版本
//! - [ ] 生成真实世界的概率

mod browser;
mod os_version;

use std::fmt::Display;

use crate::{browser::Browser, os_version::OS};

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

pub struct UserAgent {
    product: Product,
    os_ver: OS,
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
        let mut rng = rand::thread_rng();
        UserAgent {
            product: Product::Mozilla,
            os_ver: OS::mobile(&mut rng),
            browser: rand::random(),
        }
    }
    pub fn pc() -> Self {
        let mut rng = rand::thread_rng();
        UserAgent {
            product: Product::Mozilla,
            os_ver: OS::pc(&mut rng),
            browser: rand::random(),
        }
    }

    pub fn custom(os_ver: OS, browser: Browser) -> Self {
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
    use rand::Rng;
    #[test]
    fn it_works() {
        for _ in 0..100 {
            println!("{}", UserAgent::random().to_string());
        }
    }

    #[test]
    fn os_check() {
        for _ in 0..100 {
            println!("Mobile:{}", UserAgent::mobile().to_string());
            println!("Pc:{}", UserAgent::pc().to_string());
        }
    }

    #[test]
    fn os_custom() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            println!("{}", UserAgent::custom(OS::Android, Browser::Chrome));
            println!(
                "{}",
                UserAgent::custom(OS::Android, Browser::random(&mut rng))
            );
            println!(
                "{}",
                UserAgent::custom(OS::mobile(&mut rng), Browser::random(&mut rng))
            );
        }
    }
}
