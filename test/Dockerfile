FROM rust:1.69-buster as builder

ARG APP=/usr/src/app
ARG APP_NAME=request_logger_test

# We create an empty project, and copy toml files to build and cache dependencies
RUN USER=root cargo new --bin ${APP_NAME}
WORKDIR ./${APP_NAME}
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/${APP_NAME}*

# Add our project and build it with cached dependencies
ADD . ./
RUN cargo build --release

# Setup our runtime image
FROM debian:buster-slim

ARG APP=/usr/src/app
ARG APP_NAME=request_logger_test

# Install some required packages using apt, ca-certificates and tzdata, a good baseline for web apps
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

# Create non root user to run the app
ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /${APP_NAME}/target/release/${APP_NAME} ${APP}/${APP_NAME}
COPY Rocket.toml ${APP}/Rocket.toml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./request_logger_test"]