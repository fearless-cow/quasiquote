pub(crate) mod quasiquote;

#[allow(unused_macros)]
macro_rules! dbgfd {
    ($fd:literal, $($tail:tt)+) => {
        {
            use std::io::Write;
            let mut file = unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd($fd) };
            write!(file, $($tail)+).unwrap();
        }
    };
}

#[allow(unused_variables)]
#[proc_macro]
pub fn quasiquote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = quasiquote::expand(input.into());
    output.into()
}
