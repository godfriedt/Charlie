# Charlie

- Charlie will become a C2 server written in Rust 
- It is currently WIP and currently acts as a reverse shell

- TO DO:
    1. multiple connections and threads for each connection
    2. when i have implemented multiple targets and target selection, you will boot into a "hub" where the only commands will be "charlie" commands and where you cannot run shell commands because you have not specified a target. once you select target, you can run OS commands without needing to preface them with shell, and can run charlie commands by prefacing them with charlie

run with `cargo +nightly run`
must install linux nightly toolchain to run on linux
