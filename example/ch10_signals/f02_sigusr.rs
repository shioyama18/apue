use libc::{c_int, pause, signal, SIGUSR1, SIGUSR2, SIG_ERR};

extern "C" fn sig_usr(signo: c_int) {
    match signo {
        SIGUSR1 => println!("received SIGUSR1"),
        SIGUSR2 => println!("received SIGUSR2"),
        _ => panic!("Received signal {}", signo),
    }
}

fn main() {
    unsafe {
        if signal(SIGUSR1, sig_usr as usize) == SIG_ERR {
            eprintln!("can't catch SIGUSR1");
        }

        if signal(SIGUSR2, sig_usr as usize) == SIG_ERR {
            eprintln!("can't catch SIGUSR2");
        }

        loop {
            pause();
        }
    }
}
