FROM rust:1

WORKDIR /app/src
COPY . .

RUN cargo build --release

ENV DATABASE_URL=/data/users.db
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

CMD ["/app/src/start_server.sh"]
