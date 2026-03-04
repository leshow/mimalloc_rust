#![allow(
    bad_style,
    unused_imports,
    unused_macros,
    clippy::all,
    function_casts_as_integer
)]

use libmimalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
