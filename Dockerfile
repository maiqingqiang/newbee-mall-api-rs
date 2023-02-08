FROM rust:1.67 as builder
WORKDIR /usr/src/newbee-mall-api-rs
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list
RUN apt-get update && apt-get install -y libmariadb-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/newbee-mall-api-rs /usr/local/bin/newbee-mall-api-rs
CMD ["newbee-mall-api-rs"]