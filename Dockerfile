FROM messense/rust-musl-cross:x86_64-musl as chef

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /user

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /user/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /user/target/x86_64-unknown-linux-musl/release/user /user
ENTRYPOINT ["/user"]
EXPOSE 3000

#ENV DEV_DATABASE_URL="postgres://freelancify_dev:dev@localhost:5432/freelancify_user_service_dev?sslmode=disable"
#ENV DEV_MONGODB_URL="mongodb+srv://freelancify-dev:rxOred%40123@cluster0.8rpokrd.mongodb.net/freelancify?retryWrites=true&w=majority"

