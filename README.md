这是一个随机生成UserAgent的程序<初版>。
未保证浏览器版本真实存在。不生成现实较少见的Agent
安卓系统版本 使用的中国内地一网站userAgent系统 top n

# Quick Start
```no run
    for _ in 0..100 {
        println!("{}", UserAgent::random().to_string());
    }
```

# Todo
- [ ] 生成Sec-CH-UA
- [ ] 更多统计信息收集,提取系统版本 和 浏览器真实版本
- [ ] 生成真实世界的概率