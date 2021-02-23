use libc::mode_t;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR};

pub const FILE_MODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;
