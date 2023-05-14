pub(crate) mod quasiquote;

#[cfg(debug_assertions)]
macro_rules! dbg_macro {
    ($fmt:literal, $i:expr) => {{
        use std::io::Write;
        if let Ok(path) = std::env::var("DEBUG_MACRO") {
            let mut file = std::fs::File::create(path).unwrap();
            write!(file, $fmt, $i).unwrap();
        }
    }};
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_macro {
    (fmt => $fmt:literal, file => $file:expr, $i:expr) => {{
        ()
    }};
}

#[allow(unused_variables)]
#[proc_macro]
pub fn quasiquote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = quasiquote::expand(input.into());
    dbg_macro!("pub fn f() {{{}}}", output.to_string());
    output.into()
}
