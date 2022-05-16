#[macro_use]
extern crate quote;
extern crate proc_macro2;
extern crate proc_macro;
extern crate syn;
mod gl_binding;
mod utils;

use proc_macro::TokenStream;
use syn::DeriveInput;
use std::time::{Duration, Instant};

#[macro_export]
#[proc_macro_attribute]
pub fn timed(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input_parsed = syn::parse(input).unwrap();
    
    let syn::ItemFn { attrs, vis, sig, block } = input_parsed;
    let stmts = &block.stmts;

    let result = quote! {
        #(#attrs)* #vis #sig {
            use std::time::{Duration, Instant};
            
            let _macro_timed_lambda_ = ||{
                #(#stmts)*
            };
            
            let _macro_timed_start_ = Instant::now(); 
            let _macro_timed_result_ = _macro_timed_lambda_();
            let _macro_timed_end_ = Instant::now();
            let _macro_timed_duration_ = _macro_timed_end_ - _macro_timed_start_;
            println!("{:?} duration {:?}",std::stringify!(#sig), _macro_timed_duration_);
            return _macro_timed_result_;
        }
    };
    return TokenStream::from(result);
}

#[macro_export]
#[proc_macro_attribute]
pub fn gl_error_trace(metadata : TokenStream, input: TokenStream) -> TokenStream{
    
    let input_parsed = syn::parse(input.clone()).unwrap();
    let syn::ItemFn { attrs, vis, sig, block } = input_parsed;
    
    let check_gl_error = quote!{
        match GetGlError() {
            Some(error) => {
                let caller_location = std::panic::Location::caller();
                let caller_line_number = caller_location.line();
                println!("{} glError: {}",caller_location,error);
            },
            None => {},
        };
    };
    
    return utils::WrapFunction(metadata,input,quote!{},check_gl_error);
}


#[macro_export]
#[proc_macro_derive(GlBinding)]
pub fn gl_binding(input: TokenStream) -> TokenStream {
    let binding_generator = gl_binding::GlBindingGenerator::new(input);
    let get_info_tokens = proc_macro2::TokenStream::from(binding_generator.GenerateGetInfo());
    let constructor_tokens = proc_macro2::TokenStream::from(binding_generator.GenerateConstructor());
    let binding_tokens = proc_macro2::TokenStream::from(binding_generator.GenerateGlBind());

    return TokenStream::from(
        quote!{
            #constructor_tokens
            #get_info_tokens
            #binding_tokens
        }
    );
}

#[macro_export]
#[proc_macro_derive(asset_index)]
pub fn asset_index(input: TokenStream) -> TokenStream {
    return TokenStream::from(quote!{
        
    })
} 
