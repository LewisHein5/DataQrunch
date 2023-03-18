FROM rust
#FROM ubuntu

WORKDIR /app
COPY . .

RUN apt update
RUN apt install -y redis-server

RUN cargo build
RUN mkdir /datasets
#RUN useradd gablorp
#USER gablorp

ENTRYPOINT ["cargo", "run"]
#ENTRYPOINT ["netcat", "-l", "8080"]