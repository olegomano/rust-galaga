extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::DeriveInput;


#[proc_macro_attribute]
pub fn timed(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input_parsed = syn::parse2(input);
    
    let syn::ItemFn { attrs, vis, sig, block } = input_parsed;
    let stmts = &block.stmts;

    let result = quote! {
        #(#attrs)* #vis #sig {
            print!("Inserted Print Statement"); 
            #(#stmts)*
        }
    };
    return TokenStream::from(result);
}
