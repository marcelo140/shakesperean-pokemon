FROM rust:1.43

ADD . /tmp/shakesperean-pokemon

CMD cargo install --path /tmp/shakesperean-pokemon
