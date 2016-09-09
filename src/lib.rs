extern crate libc;

#[macro_use]
extern crate nom;

extern crate tls_parser;

const SURICATA_RUST_MAGIC : u32 = 0x1234;

pub use common::*;
#[macro_use]
pub mod common;

pub use tls::*;
pub mod tls;

#[no_mangle]
pub extern "C" fn rusticata_init(config: &'static mut SuricataConfig) -> i32 {
    // println!("Hello from rust");

    assert!(std::ptr::null_mut() != config);
    unsafe { suricata_config = Some(config) };

    assert_eq!(config.magic,SURICATA_RUST_MAGIC);

    SCLogError!("test error");
    SCLogInfo!("test info");
    SCLogDebug!("test debug");

    0
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
