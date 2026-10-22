mod perms;

pub use perms::{Perm, Perms};
pub use potk_macros::{EnumCode, enum_int, legacy, perms};

#[allow(unused_extern_crates)]
extern crate self as potk;

pub trait Command {
    fn parse(args: std::env::Args) -> Self;
    fn help() -> String;
}

pub fn command<T: Command + Default>() -> T {
    let mut args = std::env::args();
    loop {
        let Some(arg) = args.next() else { break T::default() };
        if arg == "-c" {
            break T::parse(args);
        }
    }
}
