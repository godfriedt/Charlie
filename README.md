# Charlie

- Charlie will become a C2 server written in Rust 
- It is currently WIP and currently acts as a reverse shell

- TO DO:
    1. multiple connections and threads for each connection
    3. command persistence. e.g. `cd /` actually changes the directory
    4. when i have implemented multiple targets and target selection, you will boot into a "hub" where the only commands will be "charlie" commands and where you cannot run shell commands because you have not specified a target. once you select target, you can run OS commands without needing to preface them with shell, and can run charlie commands by prefacing them with charlie
    5. spawn a shell session that we exist inside of instead of running commands over and over again from the same context
