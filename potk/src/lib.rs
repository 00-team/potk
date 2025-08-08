mod perms;

pub use perms::{Perm, Perms};
pub use potk_macros::{EnumCode, enum_int, legacy, perms};

#[allow(unused_extern_crates)]
extern crate self as potk;
