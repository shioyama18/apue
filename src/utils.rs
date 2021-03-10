use libc::{c_int, WCOREDUMP, WEXITSTATUS, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WSTOPSIG, WTERMSIG};

pub fn pr_exit(status: c_int) {
    if WIFEXITED(status) {
        println!("normal termination, exit status = {}", WEXITSTATUS(status));
    } else if WIFSIGNALED(status) {
        println!(
            "abnnormal termination, signal number = {}{}",
            WTERMSIG(status),
            if WCOREDUMP(status) {
                " (core file generated)"
            } else {
                ""
            }
        );
    } else if WIFSTOPPED(status) {
        println!("child stopped, signal number = {}", WSTOPSIG(status));
    }
}
