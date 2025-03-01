FROM rust:bookworm AS build

WORKDIR /home
RUN ["git", "clone", "--depth", "1", "--branch", "1.1.5", "https://github.com/Deadelineurz/rustupolis.git"]
WORKDIR /home/rustupolis
RUN ["cargo", "build", "-r"]

FROM debian:bookworm AS runtime

RUN ["apt-get", "update"]
RUN ["apt-get", "install", "ssh", "-y"]

COPY --from=build /home/rustupolis/target/release/rustupolis /usr/bin/rustupolis
COPY ./sshd_config /etc/ssh/sshd_config

RUN ["useradd", "rustupolis", "-s", "/usr/bin/rustupolis"]
RUN ["passwd", "-d", "rustupolis"]
RUN ["service", "ssh", "start"]
CMD ["/usr/sbin/sshd", "-D"]
