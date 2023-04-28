//! 这是一个随机生成UserAgent的程序<初版>。
//! 未保证浏览器版本真实存在。不生成现实较少见的Agent
//! 安卓系统版本 使用的中国内地一网站userAgent系统 top n
//!
//! # Quick Start
//! ```no run
//!     for _ in 0..100 {
//!         println!("{}", UserAgent::random().to_string());
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        for _ in 0..100 {
            println!("{}", UserAgent::random().to_string());
        }
    }

    #[test]
    fn get_OS() {}
}
