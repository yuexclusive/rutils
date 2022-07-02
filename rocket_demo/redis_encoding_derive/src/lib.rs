use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ToRedisArgs)]
pub fn to_redis_args_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_to_redis_args(&ast)
}

fn impl_to_redis_args(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ToRedisArgs for #name {
            fn write_redis_args<W>(&self, out: &mut W)
            where
                W: ?Sized + redis::RedisWrite {
                let res = &bincode::serialize(self).unwrap();
                out.write_arg(res)
            }
        }
    };
    gen.into()
}

