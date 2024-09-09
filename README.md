This is a program that randomly generates UserAgents.

这是一个随机生成UserAgent的程序

## Data From . 数据整理自
[whatmyuseragent.com](https://whatmyuseragent.com/)

[user-agents.net](https://user-agents.net/)

[useragentstring.com](https://useragentstring.com)
# Quick Start
```rust
println!("{}", UserAgent::random().to_string());
```
## Mobile and computer designated.手机电脑指定
```rust
// Random mobile agent. [随机手机端agent]
println!("Mobile:{}", UserAgent::mobile().to_string());
// Random Desktop agent.随机PC端
println!("Pc:{}", UserAgent::pc().to_string());
```
## Fully customizable.完全自定义
> `Browser`、`Devices`、`DesktopDevice`、`MobileDevice` impl rand::distributions::Standard
```rust
println!(
    "custom Mobile Iphone Chrome:{}",
    UserAgent::custom(Devices::Mobile(MobileDevice::Iphone), Browser::Chrome)
);
//first
println!(
    "custom Desktop Windows random browser :{}",
    UserAgent::custom(Devices::Desktop(DesktopDevice::Windows), rand::random())
);
//two
let mut rng = rand::thread_rng();
println!(
    "custom random Mobile & random browser :{}",
    UserAgent::custom(Devices::Mobile(rand::random()), Browser::random(&mut rng))
);
```
# Todo
- [ ] create Sec-CH-UA