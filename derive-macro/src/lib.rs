use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(Describe)]
pub fn describe(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut keys = Vec::new();
    let mut idents = Vec::new();
    let mut types = Vec::new();

    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        keys.push(quote! { #literal_key_str });
        idents.push(&field.ident);
        types.push(type_name.to_token_stream());
    }

    let expanded = quote! {
        impl #struct_name {
            fn describe() {
                let mut fields = Vec::new();
                #(
                    println!(
                        "key={key}, type={type_name}",
                        key = #keys,
                        type_name = stringify!(#types)
                    );
                    fields.push(#keys);
                    fields.push(stringify!(#types));
                )*

                println!("{:?}", fields);
            }
        }
    };
    expanded.into()
}
