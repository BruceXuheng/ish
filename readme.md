
# Rust TcpServer & Android Client & Protobuf 3

### 环境

开发系统: Windows
编码IDE: RustRover , AndroidStudio
语言 : Rust , Kotlin


### 说明
> android客户端代码仅提供了一个样例代码文件

demo实现:
    1. windows起5139 Tcp服务, 等待接收连接, 解析字节反序列化 Protobuf3 对象
    2. android客户端通过adb端口映射, 请求5139, 请求协议为Protobuf 3


### 开发步骤
##### 1. 安装 protoc
    a. github 下载 protoc (https://github.com/protocolbuffers/protobuf/releases)
    b. 设置环境变量 ,例如我将 D:\Program Files\protoc-25.0-win64\bin 设置到 path中

##### 2. 创建Rust工程
    cargo new ***

##### 3. 添加依赖至Cargo.toml
    ```
    [dependencies]
    bytes = "0.4"
    prost = "0.5"
    
    [build-dependencies]
    prost-build = "0.5"
    ```
##### 4. src中放入 mainjar_phone.proto 协议文件

##### 5. 创建build.rs
    > 我这里也是看的prost写的配置, 很遗憾我自定义到外部其他目录编译会失败
    ```
    extern crate prost_build;
    
    fn main() {
    	prost_build::compile_protos(&["src/mainjar_phone.proto"],
    	                            &["src/"]).unwrap();
    }
    ```
##### 6. cargo build

##### 7. main.rs lib.rs

##### 8. run

8.1:
``` shell
adb reverse tcp:5139 tcp:5139
```

8.2:
``` shell
cargo run
```

8.3:
启动Android 模拟客户端程序

