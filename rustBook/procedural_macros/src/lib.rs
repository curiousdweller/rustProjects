use proc_macro::{TokenStream};
use syn;
use syn::__private::ToTokens;

#[proc_macro]
pub fn do_nothing(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    eprintln!("{}", input);
    input
}

#[proc_macro_attribute]
pub fn my_attribute(input: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("Attribute Macro;{}", input);
    item
}

#[proc_macro_attribute]
pub fn config(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let res = syn::parse::<syn::ItemTrait>(input).expect("LUL");
    eprintln!("{:#?}", res.generics.where_clause);
    let mut ts = TokenStream::new();
    syn::ItemTrait::into_token_stream(res).into()
}