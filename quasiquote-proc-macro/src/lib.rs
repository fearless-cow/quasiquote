#![deny(clippy::pedantic, clippy::use_self)]
#![allow(clippy::missing_panics_doc)]

pub(crate) mod quasiquote;

#[allow(unused_variables)]
#[proc_macro]
pub fn quasiquote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quasiquote::expand(input.into()).into()
}

#[proc_macro]
#[cfg(debug_assertions)]
pub fn quasiquote_dbg(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use std::{env, fs::File, io::Write};
    let mut dbg_input_file = File::create(
        env::var("QUASIQUOTE_DBG_INPUT")
            .as_deref()
            .unwrap_or("/dev/null"),
    )
    .unwrap();
    let mut dbg_ouput_file = File::create(
        env::var("QUASIQUOTE_DBG_OUTPUT")
            .as_deref()
            .unwrap_or("/dev/null"),
    )
    .unwrap();
    let mut display_output_file = File::create(
        env::var("QUASIQUOTE_DISPLAY_OUTPUT")
            .as_deref()
            .unwrap_or("/dev/null"),
    )
    .unwrap();
    let input: proc_macro2::TokenStream = input.into();
    let output = quasiquote::expand(input.clone());
    write!(dbg_input_file, "{input:?}").unwrap();
    write!(dbg_ouput_file, "{output:?}").unwrap();
    write!(display_output_file, "fn f(){{{output}}}").unwrap();
    output.into()
}
