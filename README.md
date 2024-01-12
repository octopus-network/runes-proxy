# 介绍
这是一个由 [salvo-cli](https://github.com/salvo-rs/salvo-cli) 生成的项目,你可以按照以下命令来运行程序以及测试(非sqlite数据库的请先按照教程修改数据库连接串,完成数据的初始工作)。
``` shell
//运行项目
cargo run 
//运行测试
cargo test
```
# 项目目录说明
# runes-proxy
- **目录:** runes-proxy 
- *文件:* Cargo.toml         (Rust项目的依赖和配置信息)
- **目录:** config         (包含所有配置文件的文件夹)
    - **目录:** certs         (存放证书文件的目录)
        - *文件:* key.pem 
        - *文件:* cert.pem 
    - *文件:* config.yml 
- **目录:** assets         (静态资源如图片、JavaScript脚本和CSS样式表)
    - *文件:* favicon.ico 
- **目录:** src         (源代码目录)
    - **目录:** routers         (包含路由处理函数的模块)
        - *文件:* static_routers.rs 
        - *文件:* mod.rs 
        - *文件:* demo.rs 
    - **目录:** middleware         (包含中间件模块)
        - *文件:* handle_404.rs 
        - *文件:* cors.rs 
        - *文件:* mod.rs 
        - *文件:* jwt.rs 
    - *文件:* config.rs         (读取和处理应用配置的模块)
    - *文件:* app_error.rs         (提供统一错误处理的功能)
    - **目录:** utils         (包含工具函数的模块)
        - *文件:* mod.rs 
        - *文件:* rand_utils.rs 
    - **目录:** dtos         (定义数据传输对象(DTOs)的模块,用于封装和传输数据)
        - *文件:* user.rs 
        - *文件:* mod.rs 
    - *文件:* app_response.rs         (规范化返回请求)
    - *文件:* main.rs         (程序的入口点,设置和启动服务)
    - **目录:** services         (包含业务逻辑服务的模块)
        - *文件:* user.rs 
        - *文件:* mod.rs 

# 关于赛风(salvo)
你可以在 https://salvo.rs/ 📖查看salvo的文档以及更多例子,如果我们的工具帮到你,欢迎start [salvo](https://github.com/salvo-rs/salvo) 和 [salvo-cli](https://github.com/salvo-rs/salvo-cli),这将给我们很大激励。❤️