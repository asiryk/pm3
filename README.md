# pm3
A process manager for development scripts

Useful staff:

- https://docs.rs/clap/latest/clap/_derive/_cookbook/git/index.html
- https://github.com/google/tarpc/tree/master/example-service/src
- https://github.com/google/tarpc/issues/300
- https://rust-lang-nursery.github.io/rust-cookbook/os/external.html

unix domain sockets; communicate via .socket file; inter process communication (ipc)

TODO: when implementing resurrect, client shold save path where command was spawned.
TODO: don't save commands by value (like pm2). save them at least by folder either (same commands may be run in different directories).
