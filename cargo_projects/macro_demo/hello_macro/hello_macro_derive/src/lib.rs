//过程宏(proc-macro)
extern crate proc_macro; //Rust 自带 proc_macro crate，因此无需将其加到 Cargo.toml 文件的依赖中

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]//当用户在一个类型上指定 #[derive(HelloMacro)] 时，hello_macro_derive 函数将会被调用
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {//几乎在所有你能看到或创建的过程宏 crate 中都一样
    // 将 Rust 代码解析为语法树以便进行操作
    let ast = syn::parse(input).unwrap(); //unwrap,在生产代码中，则应该通过 panic! 或 expect 来

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream { //依过程宏的目的而各有不同
    let name = &ast.ident; //ident（ identifier，表示名字）
    let gen = quote! { //
        impl HelloMacro for #name { //动态实现
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));//stringify! 表达式转换为一个字符串常量
            }
        }
    };
    gen.into() //
}

/*
//Cargo.toml中有增加

[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
 */