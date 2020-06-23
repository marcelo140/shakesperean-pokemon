FROM rust:1.43

ADD . /tmp/shakesperean-pokemon

RUN cargo install --path /tmp/shakesperean-pokemon

CMD exec shakesperean-pokemon

EXPOSE 8080
