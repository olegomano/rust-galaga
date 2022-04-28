use proc_macro::TokenStream;
use proc_macro::TokenTree;

use quote::{quote, ToTokens};
use super::utils;

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
enum DataType{
    Vec2,
    Vec3,
    Vec4,
    Mat4x4,
    Int1,
    Int2,
    Int3,
    Int4,
}

impl DataType{
    pub fn ToString(&self) -> String {
        match self{
            DataType::Vec4 => return "vec4".to_owned(),
            DataType::Vec3 => return "vec2".to_owned(),
            DataType::Vec2 => return "vec2".to_owned(),
            DataType::Mat4x4 => return "mat4x4".to_owned(),
            _=> return "Unknown".to_owned(),
   
        }
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum ShaderType{
    Fragment,
    Vertex,
}


#[derive(Clone)]
pub struct GlShader{
    shader_type : ShaderType,
}


#[derive(Clone)]
pub struct GlUniform{
    data_type : DataType,
}

#[derive(Clone)]
pub struct GlAttribute{
    data_type : DataType,
}

#[derive(Clone)]
pub struct GlTexture{
}

#[derive(Clone)]
pub struct GlProgram{
}

#[derive(Clone)]
enum GlMemberEnum{
    Uniform(GlUniform),
    Attribute(GlAttribute),
    Texture(GlTexture),
    Shader(GlShader),
    Program(GlProgram),
}

impl GlMemberEnum{
    pub fn ToString(&self) -> String{
        match self{
            GlMemberEnum::Attribute(_) => return "Attribute".to_owned(),
            GlMemberEnum::Shader(_) => return "Shader".to_owned(),
            GlMemberEnum::Texture(_) => return "Texture".to_owned(),
            GlMemberEnum::Uniform(_) => return "Uniform".to_owned(),
            _ => return "".to_owned(),
        }
    }
    
    pub fn Info(&self) -> String{
        let mut result : String = "".to_owned();
        result.push_str(&self.ToString());
        result.push_str(" ");
    
        let data_type = match self{
            GlMemberEnum::Uniform(u) => u.data_type.ToString(),
            GlMemberEnum::Attribute(a) => a.data_type.ToString(),
            _=> "".to_owned(),
        };
        result.push_str(&data_type);
        return result;
    } 

    pub fn BindFunction(&self, name : &str) -> proc_macro2::TokenStream {
        match self{
            GlMemberEnum::Uniform(u) => return self.UniformBindFunction(name),
            GlMemberEnum::Attribute(u) => return self.AttributeBindFunction(name),
            GlMemberEnum::Texture(u) => return self.AttributeBindFunction(name),
            _ => return quote!{}
        }
    }
    
    fn UniformBindFunction(&self,name : &str) -> proc_macro2::TokenStream {
        let program_id = utils::ToMemberIdent("program_id"); 
        return quote!{
            GetUniformLocation(#program_id,#name)
        }
    }

    fn AttributeBindFunction(&self, name : &str) -> proc_macro2::TokenStream {
        let program_id = utils::ToMemberIdent("program_id"); 
        return quote!{
            GetAttributeLocation(#program_id,#name)
        }
    }
}

#[derive(Clone)]
pub struct GlMember{
    member_info : utils::MemberInfo,
    gl_member : GlMemberEnum,
}

impl GlMember{
    pub fn new(info : &utils::MemberInfo) -> Option<Self>{
        let tokens : Vec<&str> = info.display_name.split("_").collect();
        if tokens.len() == 1{
            return None
        }
    
        let member : GlMemberEnum = match tokens[0]{
            "a" => {
                let data_type = GetDataType(tokens[1]);
                match data_type{
                    Some(d) => {
                        GlMemberEnum::Attribute(
                            GlAttribute{
                                data_type : d,
                            })
                        }
                    None =>{
                        return None
                    }
                }
            },
            "u" => {
                match GetDataType(tokens[1]){
                    Some(d) => {
                        GlMemberEnum::Uniform(
                            GlUniform{
                                data_type : d,
                        })
                        
                    }
                    None =>{
                        return None
                    }
                 }
            },
            "t" => GlMemberEnum::Texture(GlTexture{}),
            _ => {return None},
        };
        return Some(Self{
            member_info : info.clone(),
            gl_member : member,
        })
    }

    pub fn Info(&self) -> String{
        return self.gl_member.Info();
    }
}

fn GetDataType(input : &str) -> Option<DataType>{
    match input{
        "vec4" => {return Some(DataType::Vec4)},
        "vec3" => {return Some(DataType::Vec3)},
        "vec2" => {return Some(DataType::Vec2)},
        "mat4x4" => {return Some(DataType::Mat4x4)},
        _ => {return None},
    }
}


pub struct GlBindingGenerator{
    gl_members : Vec<GlMember>,
    info : utils::StructInfo,
}


impl GlBindingGenerator{ 
    pub fn new(tokens : TokenStream) -> Self{
        let struct_info = utils::StructInfo::new(tokens).expect("");
        let mut gl_members : Vec<GlMember> = Vec::new();
        for member in struct_info.Members().iter() {
            let parsed_member = GlMember::new(member);
            match parsed_member{
                Some(m) => gl_members.push(m),
                None => {}
            }
        }
        return Self{
            gl_members : gl_members,
            info : struct_info,
        }
    }

    pub fn GenerateConstructor(&self) -> TokenStream {
        let mut member_init_tokens = quote!{};  
        let struct_ident = &self.info.ident;
        
        for member in self.info.Members() {
            let member_ident = &member.ident;
            member_init_tokens.extend(quote!{
                #member_ident : -1,
            });
        }

        return TokenStream::from(quote!{
            impl #struct_ident{
                pub fn new(vert_shader : &str, frag_shader : &str) -> Option<Self> {
                    let mut result =  Self{
                        #member_init_tokens
                    };
                    match CompileShader(vert_shader,frag_shader){
                        Ok(o) => result.program_id = o, 
                        Err(e) => {
                            println!("Shader compilation failed: {}",e);
                            return None;
                        }
                    }
                    result.GlBind();
                    return Some(result);
                }
            }
        });
    }

    pub fn GenerateGetInfo(&self) -> TokenStream{
        let struct_ident = &self.info.ident;
        let struct_info = self.info.GenerateGetInfo();
        
        
        let mut member_info_string : String = "".to_owned();
        for member_info in self.gl_members.iter(){
            member_info_string.push_str(&member_info.Info());
            member_info_string.push_str("\n");
        }

        let mut member_info_tokens = quote!{
            pub fn GetGlInfo(&self) -> String{
                return (#member_info_string).to_string()
            }
        };
        
        return TokenStream::from(quote!{
            impl #struct_ident{
                #struct_info
                #member_info_tokens
            }
        });
    }

    pub fn GenerateGlBind(&self) -> TokenStream{
        let mut binding_tokens = quote!{};  
        let struct_ident = &self.info.ident;
            
        for member in self.gl_members.iter() {
            let bind_function = member.gl_member.BindFunction(&member.member_info.display_name);
            let member_name = utils::ToMemberIdent(&member.member_info.display_name);
            binding_tokens.extend(quote!{
                match #bind_function{
                    Some(x) => #member_name = x,
                    None => println!("Failed to bind glAttribute {}",#member_name),
                }
            });
        }

        return TokenStream::from(quote!{
            impl #struct_ident{
                pub fn GlBind(&mut self){
                  #binding_tokens
                }
            }
        });
    }
}

