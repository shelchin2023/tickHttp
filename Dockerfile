# ==========================
# Build Stage
# ==========================
# 使用官方的 Rust 编译环境作为基础镜像
FROM rust:1.82 AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

# 设置工作目录
WORKDIR /usr/src/app

# 将当前目录的文件全部拷贝进容器
COPY . .

# 编译项目为 Release 版本
RUN cargo build --release


# ==========================
# Final Stage
# ==========================
# 使用兼容的新基础镜像（较新的 GLIBC）
FROM debian


# 安装运行时所需的依赖
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从构建阶段复制编译好的二进制文件到最终镜像
COPY --from=builder /usr/src/app/target/release/tickHttp ./tickHttp

# 配置启动容器时的命令，运行编译好的 Rust 服务
ENTRYPOINT ["./tickHttp"]