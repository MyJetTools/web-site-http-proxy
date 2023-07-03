FROM rust:slim
COPY ./target/release/web-site-http-proxy ./target/release/web-site-http-proxy
ENTRYPOINT ["./target/release/web-site-http-proxy"]