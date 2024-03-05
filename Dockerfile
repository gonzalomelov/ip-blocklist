FROM rust:1.63.0 as builder
WORKDIR /usr/src/ip-blocklist
COPY . .
RUN curl --compressed https://raw.githubusercontent.com/stamparm/ipsum/master/ipsum.txt | grep -v "#" | cut -f 1 > ips.csv
RUN cargo install --path .
 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ip-blocklist /usr/local/bin/ip-blocklist
COPY --from=builder /usr/src/ip-blocklist/ips.csv /usr/src/ip-blocklist/ips.csv
CMD ["ip-blocklist", "/usr/src/ip-blocklist/ips.csv"]