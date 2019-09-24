
## 环境搭建

1. 用 pki/docker/Dockerfile 构建镜像 `openssl:alpine-3.10`
    ```bash
    cd pki/docker 
    docker build -t openssl:alpine-3.10
    ```
2. 构建 PKI 系统
    ```bash
    # 生成的文件包括
    #   1. CA 的证书 ca.cert 和 RSA 私钥 ca.key
    #   2. 服务器的证书 server.cert 和 RSA 私钥 server.key
    # 且服务器证书经 CA 签名
    docker run --rm -it -v ${PWD}:/workspace -w /workspace openssl:alpine-3.10 sh pki.sh
    ```
3. 分发证书到客户端和服务器
    ```bash
    # 服务器私钥和证书分发给服务器
    cp server.key ../server/
    cp server.cert ../server/

    # 客户端则接收 CA 证书即可
    cp ca.cert ../client
    ```

## 运行
1. 服务器 
```bash
cd server 
cargo run

# 样例输出
hello world%
```
2. 客户端
```bash
cd client 
cargo run

# 样例输出
OK from server
```

## 坑
- macOS 上的 openssl 生成的证书版本有问题
