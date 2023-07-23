FROM rust:1.69-buster AS builder
WORKDIR /backend

COPY ./backend .
RUN cargo build --release

# =========================
FROM debian:buster-slim
WORKDIR /homeland

RUN apt-get update
RUN apt-get install curl -y
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install -y nodejs
RUN npm i pm2 -g

COPY ./frontend ./frontend
COPY ./backend/.env .
COPY ./ecosystem.config.cjs .

COPY --from=builder /backend/target/release/backend .
RUN cd frontend && npm install --registry=https://registry.npmmirror.com && npm run build && cd ..

# ENV APP_USER=appuser
# ARG APP=/homeland
# RUN groupadd $APP_USER \
#     && useradd -g $APP_USER $APP_USER \
#     && mkdir -p ${APP}
# RUN chown -R $APP_USER:$APP_USER ${APP}

EXPOSE 3000
EXPOSE 9000

CMD ["pm2-runtime", "ecosystem.config.cjs"]

