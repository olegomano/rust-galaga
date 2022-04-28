use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::{quote, ToTokens};

pub fn ToMemberIdent(name : &str) -> proc_macro2::TokenStream{
    let ident_str = format!("self.{}", name);
    let ident_token = ident_str.parse::<::proc_macro2::TokenStream>().unwrap();
    return ident_token;
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct MemberInfo{
    pub display_name : String,
    pub ident : syn::Ident,
}

impl MemberInfo{
    pub fn new(field : &syn::Field) -> Option<Self>{
        let ident : syn::Ident = field.ident.as_ref().expect("").clone();
        return Some(Self{
            ident : ident,
            display_name : field.ident.as_ref().expect("").to_string(),
        })
    }

    pub fn InfoString(&self) -> &str {
        return &self.display_name;
    }
}

pub struct StructInfo{
   pub display_name : String,
   pub ident : syn::Ident,
   members : Vec<MemberInfo>,
}

impl StructInfo{
    pub fn new(input : TokenStream) -> Option<Self>{
        let struct_info : syn::ItemStruct = syn::parse(input).unwrap();
        let mut members : Vec<MemberInfo> = Vec::new(); 
        for i in struct_info.fields.iter() {
            match MemberInfo::new(i){
                Some(info) => members.push(info),
                None => {}
            }
        
        }
        return Some(Self{
            ident : struct_info.ident.clone(),
            display_name : struct_info.ident.to_string(),
            members : members,
        })
    }

    pub fn Members(&self) -> &Vec<MemberInfo> {
        return &self.members;
    }

    pub fn GenerateGetInfo(&self) -> proc_macro2::TokenStream {
        let mut name : String = self.display_name.to_owned();
        name.push_str("\n");

        for member in self.members.iter() {
            name.push_str(&member.display_name);
            name.push_str("\n");
        }

        let info = quote!{
            pub fn GetInfo(&self) -> String{
                return #name.to_string();
            }
        };
        return info;

    }
}
