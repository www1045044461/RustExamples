use core::panic;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    parse, parse_macro_input, punctuated::PairsMut, token::Question, Block, Data, DeriveInput,
    Fields, ItemFn, Meta, MetaList,
};
extern crate proc_macro;

/// 一个用于函数的属性,生成新的函数记录被调用日志
/// 能通过编译和执行,但是对_attr没有处理
#[proc_macro_attribute]
pub fn callee_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let meta = parse_macro_input!(_attr as syn::Meta); //这个是错的?
    let code = parse_macro_input!(item as ItemFn);

    let fn_name = &code.sig.ident;
    let fn_params = &code.sig.inputs;
    let fn_ret = &code.sig.output;

    let bodys = &code.block.stmts;

    let gen_code = quote! {
        fn #fn_name(#fn_params) #fn_ret {
            println!("enter function:{}",stringify!(#fn_name));
            #(#bodys)*
        }
    };
    gen_code.into()
}

/// 一个用于函数的属性,生成新的函数记录被调用日志
/// 在原始的基础上构建"标识符"加入日志输出中
/// 结果:只能识别带有Id=1的别的都不行
#[proc_macro_attribute]
pub fn callee_log1(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(_attr as syn::Meta); //这个是错的?
    let code = parse_macro_input!(item as ItemFn);

    let fn_name = &code.sig.ident;
    let fn_params = &code.sig.inputs;
    let fn_ret = &code.sig.output;

    let bodys = &code.block.stmts;

    //构建新的构建符
    let mut _str = Ident::new("default", Span::call_site());

    if let Meta::Path(_path) = meta {
        _str = Ident::new("path", Span::call_site());
    } else if let Meta::List(_list) = meta {
        _str = Ident::new("List", Span::call_site());
    } else if let Meta::NameValue(_nv) = meta {
        _str = Ident::new("NameValue", Span::call_site());
    } else {
    }

    let gen_code = quote! {
        fn #fn_name(#fn_params) #fn_ret{
            println!("enter function type:{} name:{}",stringify!(#_str),stringify!(#fn_name));
            #(#bodys)*
        }
    };

    gen_code.into()
}

/// 根据传入的属性创建新的函数;比如创建所有函数的Debug版本
/// 但是目前只实现了Path类型的,别的类型传不进来!
#[proc_macro_attribute]
pub fn callee_create(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(_attr as syn::Meta); //这个是错的?
    let code = parse_macro_input!(item as ItemFn);

    let fn_name = &code.sig.ident;
    let fn_params = &code.sig.inputs;
    let fn_ret = &code.sig.output;

    let bodys = &code.block.stmts;

    //构建新的构建符
    let mut _str = Ident::new("default", Span::call_site());

    if let Meta::Path(_path) = meta {
        _str = _path.get_ident().unwrap().to_owned(); //直接获取path的标识符
    } else if let Meta::List(_list) = meta {
        _str = Ident::new("List", Span::call_site()); //无效
    } else if let Meta::NameValue(_nv) = meta {
        _str = Ident::new("NameValue", Span::call_site()); //无效
    } else {
    }

    let new_name_str = format!("{}_{}", _str.to_string(), fn_name.to_string());

    let new_fn_name = Ident::new(&new_name_str, Span::call_site());

    let gen_code = quote! {
        fn #new_fn_name(#fn_params) #fn_ret{
            println!("enter function type:{} name:{}",stringify!(#_str),stringify!(#new_fn_name));
            #(#bodys)*
        }
    };

    gen_code.into()
}

///
/// 属性宏添加然后调用
#[proc_macro_attribute]
pub fn before_caller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident; //函数名
    let inputs = &input_fn.sig.inputs;
    let stmts = &input_fn.block.stmts;
    let returns = &input_fn.sig.output;

    let gen_code = quote! {
        fn #fn_name(#inputs) #returns {
            println!("Call function:{}",stringify!(#fn_name));
            println!("Attr:{:?}",stringify!(#inputs));
            #(#stmts)*
        }
    };

    gen_code.into()
}

#[proc_macro]
pub fn excute_block(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block); //将code代码段转换为语法元素
                                                    //利用输入的语法元素转换为新的语法元素
    let expand = quote! {
        fn execute() {
         #block
        }
    };

    //将新的语法元素转换为新的code代码段!
    TokenStream::from(expand)
}

/// 疑问?:AnswerFn是否要提前定义?
#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer()-> u32 {45}".parse().unwrap()
}

/// 为宿主类实现静态方法struct_hello
#[proc_macro_derive(StructHelloFn)]
pub fn derive_struct_hello_fn(token: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(token as DeriveInput);
    let name = &ast.ident;

    let gen_code = quote! {
        impl #name {
            pub fn struct_hello(){
                println!("hello {}",stringify!(#name));
            }
        }
    };

    gen_code.into()
}

#[proc_macro_derive(ObjectShowTrait)]
pub fn derive_object_show_trait(token: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(token as DeriveInput);
    let struct_name = &ast.ident;

    let str_struct_name = struct_name.to_string();

    //获取函数体
    let fields = match &ast.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("this macro can only be used on structs"),
    };

    let print_fn = generate_print_fn(fields);

    //结构体名字
    let expanded = quote! {
        impl #struct_name {
            pub fn print_fields(&self){
                println!("{}{{",stringify!(#struct_name));
                //将宏中语法元素转换为字符常量
                #print_fn
                println!("}}");
            }
        }
    };

    TokenStream::from(expanded)
}

/// 将所有属性的集合生成TokenStream
/// 没有宏修饰符的函数不能定义在宏libs中
fn generate_print_fn(fields: &Fields) -> proc_macro2::TokenStream {
    let mut print_statements = vec![];

    //遍历所有字段生成打印语句
    for field in fields.iter() {
        let field_name = &field.ident; //字段名
        let field_name_str = field_name.as_ref().map(|ident| ident.to_string()).unwrap();
        //获取字段名表示,类型由语法元素变换为string
        let print_stmt = quote! {
            println!("\t{}:{:?}",#field_name_str,&self.#field_name);
        };
        print_statements.push(print_stmt);
    }

    //将所有打印语句组合在一起
    quote! {
        #(#print_statements)*
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
