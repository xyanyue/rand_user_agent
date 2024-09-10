use std::fmt::Display;

use crate::config::{
    ANDROID_VER, GALAXY, HARMONYOS, HUAWEI, IPHONE, LINUX_VER, MACINTOSH_VER, REDMI, WIN_BIT,
    WIN_NT, XIAOMI,
};
use rand::{distributions::Standard, prelude::Distribution, Rng};
pub enum Devices {
    Desktop(DesktopDevice),
    Mobile(MobileDevice),
}
impl Devices {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0..=5) {
            0 => Devices::Desktop(rand::random()),
            1 => Devices::Desktop(rand::random()),
            2 => Devices::Desktop(rand::random()),
            3 => Devices::Mobile(rand::random()),
            4 => Devices::Mobile(rand::random()),
            _ => Devices::Mobile(rand::random()),
        }
    }

    pub fn mobile() -> Self {
        Devices::Mobile(rand::random())
    }
    pub fn pc() -> Self {
        Devices::Desktop(rand::random())
    }
}
impl Distribution<Devices> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Devices {
        Devices::random(rng)
    }
}

impl Display for Devices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Devices::Desktop(desktop) => write!(f, "{}", desktop),
            Devices::Mobile(mobile) => write!(f, "{}", mobile),
        }
    }
}
pub enum MobileDevice {
    Galaxy,
    Huawei,
    HarmonyOS,
    Iphone,
    // One,
    Redmi,
    Mi,
    // Lumia,
    // Moto,
    // Nova,
    // Oppo,
}
impl MobileDevice {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0..6) {
            0 => MobileDevice::Galaxy,
            1 => MobileDevice::Huawei,
            2 => MobileDevice::HarmonyOS,
            3 => MobileDevice::Iphone,
            4 => MobileDevice::Mi,
            5 => MobileDevice::Redmi,
            _ => MobileDevice::Galaxy, // no
        }
    }
}
impl Distribution<MobileDevice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MobileDevice {
        MobileDevice::random(rng)
    }
}
impl Display for MobileDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rng = rand::thread_rng();
        match self {
            MobileDevice::Galaxy => write!(
                f,
                "(Linux; Android {}; {})",
                ANDROID_VER[rng.gen_range(0..ANDROID_VER.len())],
                GALAXY[rng.gen_range(0..GALAXY.len())],
            ),
            MobileDevice::Huawei => write!(
                f,
                "(Linux; Android {}; {})",
                ANDROID_VER[rng.gen_range(0..ANDROID_VER.len())],
                HUAWEI[rng.gen_range(0..HUAWEI.len())],
            ),
            MobileDevice::HarmonyOS => {
                write!(f, "({})", HARMONYOS[rng.gen_range(0..HARMONYOS.len())],)
            }
            MobileDevice::Iphone => {
                write!(
                    f,
                    "(iPhone; CPU iPhone OS {} like Mac OS X)",
                    IPHONE[rng.gen_range(0..IPHONE.len())]
                )
            }
            MobileDevice::Mi => {
                write!(
                    f,
                    "(Linux; Android {}; {})",
                    ANDROID_VER[rng.gen_range(0..ANDROID_VER.len())],
                    XIAOMI[rng.gen_range(0..XIAOMI.len())]
                )
            }
            MobileDevice::Redmi => write!(
                f,
                "(Linux; Android {}; {})",
                ANDROID_VER[rng.gen_range(0..ANDROID_VER.len())],
                REDMI[rng.gen_range(0..REDMI.len())]
            ),
        }
    }
}
pub enum DesktopDevice {
    Windows,
    Macintosh,
    Linux,
}
impl DesktopDevice {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0..5) {
            0 => DesktopDevice::Windows,
            1 => DesktopDevice::Windows,
            2 => DesktopDevice::Macintosh,
            3 => DesktopDevice::Macintosh,
            4 => DesktopDevice::Linux,
            _ => DesktopDevice::Linux, // no
        }
    }
}
impl Distribution<DesktopDevice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DesktopDevice {
        DesktopDevice::random(rng)
    }
}

impl Display for DesktopDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rng = rand::thread_rng();
        match self {
            DesktopDevice::Windows => write!(
                f,
                "(Windows NT {};{})",
                WIN_NT[rng.gen_range(0..WIN_NT.len())],
                WIN_BIT[rng.gen_range(0..WIN_BIT.len())],
            ),
            DesktopDevice::Macintosh => write!(
                f,
                "(Macintosh; Intel Mac OS X {})",
                MACINTOSH_VER[rng.gen_range(0..MACINTOSH_VER.len())],
            ),
            DesktopDevice::Linux => {
                write!(f, "(X11; {})", LINUX_VER[rng.gen_range(0..LINUX_VER.len())],)
            } // OS::Android => write!(
              //     f,
              //     "(Linux; Android {}; {})",
              //     ANDROID_VER[rng.gen_range(0..ANDROID_VER.len())],
              //     ANDROID_OS[rng.gen_range(0..ANDROID_OS.len())],
              // ),
              // OS::Ios => write!(f, "(iPhone; CPU iPhone OS 16_3_1 like Mac OS X)"),
        }
    }
}
