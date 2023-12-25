FROM rust:1.68-alpine as builder
RUN apk add openssl-dev musl-dev
WORKDIR /user
COPY . . 
ENV SQLX_OFFLINE true

#ENV DATABASE_URL="postgres://freelancify_dev:dev@localhost:5432/freelancify_user_service_dev?sslmode=disable"
#ENV MONGODB_URL="mongodb+srv://freelancify-dev:rxOred%40123@cluster0.8rpokrd.mongodb.net/freelancify?retryWrites=true&w=majority"

#ENV DEV_DATABASE_URL="postgres://freelancify_dev:dev@localhost:5432/freelancify_user_service_dev?sslmode=disable"
#ENV DEV_MONGODB_URL="mongodb+srv://freelancify-dev:rxOred%40123@cluster0.8rpokrd.mongodb.net/freelancify?retryWrites=true&w=majority"

RUN cargo build --release

FROM rust:1.68-alpine as runtime

WORKDIR /user 
COPY --from=builder /user/target/release/user user
COPY .env .env 
ENTRYPOINT ["./user"]
EXPOSE 8002
