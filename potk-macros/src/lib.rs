mod enum_code;
mod enum_int;
mod legacy;
mod perms;

use proc_macro::TokenStream;

/// Example:
/// ```
/// #[derive(Debug, potk_macros::EnumCode)]
/// #[enum_code(u8)]
/// pub enum Schema {
///     Model(String), // 0u8
///     Array { len: usize }, // 1u8
///     U8,  // 2u8
///     Gene // 3u8
/// }
/// ```
#[proc_macro_derive(EnumCode, attributes(enum_code))]
pub fn enum_code(code: TokenStream) -> TokenStream {
    enum_code::enum_code(code)
}

/// Example:
/// ```ignore
/// #[potk::legacy]
/// mod items {
///
///     /// the derive macros from **Base** are set for all children
///     #[derive(Debug, Serialize, Deserialize, ToSchema)]
///     pub struct Base {
///         gene: Gene,
///         is_alive: bool,
///     }
///
///     impl From<&Review> for Base {
///         fn from(value: &Review) -> Self {
///             Self {
///                 gene: value.gene,
///                 is_alive: value.is_alive(),
///             }
///         }
///     }
///
///     // child 1
///     pub struct ReviewInfo {
///         // gene: Gene,
///         // is_alive: bool,
///         user: Gene,
///     }
///
///     impl From<&Review> for ReviewInfo {
///         fn from(value: &Review) -> Self {
///             Self { user: value.user }
///         }
///     }
///
///     // child 2
///     pub struct EateryReviewInfo {
///         // gene: Gene,
///         // is_alive: bool,
///         user: Option<EateryReviewUserInfo>,
///     }
///
///     impl From<&Review> for EateryReviewInfo {
///         fn from(review: &Review) -> Self {
///             Self { user: None }
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn legacy(_args: TokenStream, code: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(code as syn::ItemMod);
    legacy::legacy(item).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// enum_ini is a two way conversion enum `<->` u16
/// default **start** is `0` and default **ty** is `u8`
/// Example:
/// ```
/// #[potk_macros::enum_int(u16)]
/// #[derive(Debug, Default, Clone, Copy)]
/// pub enum ExampleError {
///     #[default]
///     Unknown = 0,
///     UserNotFound,
///     BadPhone,
///     BadStr,
/// }
/// ```
#[proc_macro_attribute]
pub fn enum_int(args: TokenStream, code: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(code as syn::ItemEnum);
    enum_int::enum_int(item, args)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// perms is for using an array of `u8` for booleans
/// ```ignore
/// pub mod my_perms {
///     potk::perms! {
///         VIEW_USERS,  ADD_USERS,  EDIT_USERS,  DEL_USERS
///     }
/// }
///
/// let mut user_perms = [0u8; 32];
/// assert_eq!(user_perms.perm_get(my_perms::VIEW_USERS), false);
/// user_perms.perm_set(my_perms::VIEW_USERS, true);
/// assert_eq!(user_perms.perm_get(my_perms::VIEW_USERS), true);
/// ```
#[proc_macro]
pub fn perms(code: TokenStream) -> TokenStream {
    perms::perms(code)
}

fn crate_ident() -> syn::Ident {
    // let found_crate = crate_name("potk").unwrap();
    // let name = match &found_crate {
    //     FoundCrate::Itself => "potk",
    //     FoundCrate::Name(name) => name,
    // };
    ident!("potk")
}

macro_rules! ident {
    ($name:literal) => {
        syn::Ident::new($name, proc_macro2::Span::call_site())
    };
    ($name:expr) => {
        syn::Ident::new($name, proc_macro2::Span::call_site())
    };
}
pub(crate) use ident;

macro_rules! err {
    ($span:expr, $($msg:literal),*) => {
        Err(syn::Error::new($span, concat!( $($msg),* )))
    };
    ($span:expr, $msg:expr) => {
        Err(syn::Error::new($span, $msg))
    };
}

pub(crate) use err;
