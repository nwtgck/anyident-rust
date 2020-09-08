extern crate proc_macro;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold("".to_owned(), |s, b| s + &format!("{:x}", b) )
}

#[proc_macro]
pub fn i(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // TODO: join all spans
    let span = item.clone().into_iter().map(|x| x.span()).collect::<Vec<_>>()[0];
    let new_str = format!("hex_{:}", hex(item.to_string().as_bytes()));
    let ident = proc_macro::TokenTree::Ident(proc_macro::Ident::new(&new_str, span));
    proc_macro::TokenStream::from(ident)
}
