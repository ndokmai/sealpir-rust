use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PirQuery {
    pub query: Vec<u8>,
    pub num: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PirReply {
    pub reply: Vec<u8>,
    pub num: u32,
}

pub mod client;
pub mod server;

#[cfg(feature = "suppress-stdout")]
fn output_log_info(stdout_buf: Option<&mut gag::BufferRedirect>) {
    use std::io::Read;
    stdout_buf.map(|b| {
        let mut stdout_output = String::new();
        b.read_to_string(&mut stdout_output).unwrap();
        log::info!("{}", stdout_output);
    });
}
