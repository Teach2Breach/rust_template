mod proto;
mod func;
#[macro_use]
extern crate litcrypt;
 
use_litcrypt!();

#[no_mangle]
pub extern "C" fn lib_main() {
    proto::Pick();
}