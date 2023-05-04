这是一个随机生成UserAgent的程序<初版>。
未保证浏览器版本真实存在。不生成现实较少见的Agent
安卓系统版本 使用的中国内地一网站userAgent系统 top n

# Quick Start
```rust
    for _ in 0..100 {
        println!("{}", UserAgent::random().to_string());
    }
```
 ### 手机电脑指定
 ```rust
     for _ in 0..100 {
         println!("Mobile:{}", UserAgent::mobile().to_string());
         println!("Pc:{}", UserAgent::pc().to_string());
     }
 ```
 ### 完全自定义
 ```rust
     let mut rng = rand::thread_rng();
     for _ in 0..100 {
         // 指定 Android & Chrome
         println!("{}", UserAgent::custom(OS::Android, Browser::Chrome));
         // 指定 Android & 随机浏览器
         println!(
             "{}",
             UserAgent::custom(OS::Android, Browser::random(&mut rng))
         );
         // 随机手机系统 & 随机浏览器  === UserAgent::mobile
         println!(
             "{}",
             UserAgent::custom(OS::mobile(&mut rng), Browser::random(&mut rng))
         );
     }
 ```

# Todo
- [ ] 生成Sec-CH-UA
- [ ] 更多统计信息收集,提取系统版本 和 浏览器真实版本
- [ ] 生成真实世界的概率