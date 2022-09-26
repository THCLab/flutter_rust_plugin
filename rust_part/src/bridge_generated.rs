#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
<<<<<<< Updated upstream
    non_snake_case
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
=======
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.48.0.

use crate::api::*;
use core::panic::UnwindSafe;
>>>>>>> Stashed changes
use flutter_rust_bridge::*;

// Section: imports

// Section: wire functions

<<<<<<< Updated upstream
#[no_mangle]
pub extern "C" fn wire_hello(port_: i64) {
=======
fn wire_hello_impl(port_: MessagePort) {
>>>>>>> Stashed changes
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "hello",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(hello()),
    )
}
<<<<<<< Updated upstream

// Section: wire structs

=======
>>>>>>> Stashed changes
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
<<<<<<< Updated upstream
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

=======
        (!self.is_null()).then(|| self.wire2api())
    }
}
>>>>>>> Stashed changes
// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

<<<<<<< Updated upstream
// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}

    // ----------- DUMMY CODE FOR BINDGEN ----------

    // copied from: allo-isolate
    pub type DartPort = i64;
    pub type DartPostCObjectFnType = unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool;
    #[no_mangle] pub unsafe extern "C" fn store_dart_post_cobject(ptr: DartPostCObjectFnType) { panic!("dummy code") }

    // copied from: frb_rust::support.rs
    #[repr(C)]
    pub struct WireSyncReturnStruct {
        pub ptr: *mut u8,
        pub len: i32,
        pub success: bool,
    }

    // ---------------------------------------------
    
=======
#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
>>>>>>> Stashed changes
