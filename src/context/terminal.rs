use std::io::{stderr, stdin, stdout, Stderr, Stdin, Stdout};

pub struct Terminal {
    pub stdout: Stdout,
    pub stderr: Stderr,
    pub stdin: Stdin,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            stdout: stdout(),
            stderr: stderr(),
            stdin: stdin(),
        }
    }
}
