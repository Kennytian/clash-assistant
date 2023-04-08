## Tauri + Rust 学习过程

### 一、项目结构
- public
  - 两个 svg 图标文件 
- src
  - 所有前端的代码，如：assets, css, tsx, ts 等
- src-tauri
  - icons 图标
  - src 存放 Rust 代码
  - Cargo.toml Rust 项目依赖文件
  - tauri.conf.json 工程文件，窗口大小、App 名称等
  - 其它

### 二、前端


### 三、后端
#### Rust
- derive 是一种用于自动生成代码的工具，它可以通过一些基类或结构体的属性和方法来自动生成一些代码。derive 是 Rust 标准库中的一个模块，可以通过在结构体或类中使用 derive 关键字来启用
- Result<VideoInfo, String> 表示 fn 函数操作成功就返回 VideoInfo，而操作失败就返回 String 类型的值

#### Third-part
- reqwest: 是一个简单而强大的 Rust HTTP 客户端，用于浏览器异步 HTTP 请求。支持 xmlHttpRequest, JSONP, CORS, 和 CommonJS 约束
- regex: 正则表达式
- futures-util：Futures 类似于 Javascript 中的promise
。算得上是巨无霸，它整个futures-rs内容最多的一个子类库，它依赖futures-core 、futures-task 、futures-channel、futures-io、futures-sink、futures-macro、tokio-io 等类库。  
- async-recursion: 可递归调用的异步函数
- serde: 是 Rust 的标准序列化库，而 derive 是用于在 Rust 中自动生成代码的工具。serde 和 derive 是两个不同的概念，但是它们之间有一些关联
### 四、其它
