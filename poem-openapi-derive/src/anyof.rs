use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromVariant,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Type};

use crate::{
    common_args::ExternalDocument,
    error::GeneratorResult,
    utils::{get_crate_name, get_summary_and_description, optional_literal},
};

#[derive(FromVariant)]
#[darling(attributes(oai), forward_attrs(doc))]
struct AnyOfItem {
    ident: Ident,
    fields: Fields<Type>,

    #[darling(default)]
    mapping: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(oai), forward_attrs(doc))]
struct AnyOfArgs {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: Data<AnyOfItem, Ignored>,

    #[darling(default)]
    internal: bool,
    #[darling(default)]
    discriminator_name: Option<String>,
    #[darling(default)]
    external_docs: Option<ExternalDocument>,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let args: AnyOfArgs = AnyOfArgs::from_derive_input(&args)?;
    let crate_name = get_crate_name(args.internal);
    let ident = &args.ident;
    let (title, description) = get_summary_and_description(&args.attrs)?;
    let title = optional_literal(&title);
    let description = optional_literal(&description);
    let discriminator_name = &args.discriminator_name;

    let e = match &args.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(ident, "AnyOf can only be applied to an enum.").into()),
    };

    let mut types = Vec::new();
    let mut from_json = Vec::new();
    let mut to_json = Vec::new();
    let mut mapping = Vec::new();
    let mut names = Vec::new();
    let mut schemas = Vec::new();

    let required = match &args.discriminator_name {
        Some(discriminator_name) => quote!(::std::vec![#discriminator_name]),
        None => quote!(::std::vec![]),
    };

    for variant in e {
        let item_ident = &variant.ident;

        match variant.fields.len() {
            1 => {
                let object_ty = &variant.fields.fields[0];
                let mapping_name = match &variant.mapping {
                    Some(mapping) => quote!(#mapping),
                    None => {
                        quote!(::std::convert::AsRef::as_ref(&<#object_ty as #crate_name::types::Type>::name()))
                    }
                };
                names.push(quote!(#mapping_name));

                types.push(object_ty);

                if discriminator_name.is_some() {
                    from_json.push(quote! {
                        if ::std::matches!(discriminator_name, ::std::option::Option::Some(discriminator_name) if discriminator_name == #mapping_name) {
                            return <#object_ty as #crate_name::types::ParseFromJSON>::parse_from_json(value)
                                .map(Self::#item_ident)
                                .map_err(#crate_name::types::ParseError::propagate);
                        }
                    });
                } else {
                    from_json.push(quote! {
                        if let ::std::option::Option::Some(obj) = <#object_ty as #crate_name::types::ParseFromJSON>::parse_from_json(::std::clone::Clone::clone(&value))
                            .map(Self::#item_ident)
                            .ok() {
                            return ::std::result::Result::Ok(obj);
                        }
                    });
                }

                if let Some(discriminator_name) = &discriminator_name {
                    to_json.push(quote! {
                        Self::#item_ident(obj) => {
                            let mut value = <#object_ty as #crate_name::types::ToJSON>::to_json(obj);
                            if let ::std::option::Option::Some(obj) = value.as_object_mut() {
                                obj.insert(::std::convert::Into::into(#discriminator_name), ::std::convert::Into::into(#mapping_name));
                            }
                            value
                        }
                    });
                } else {
                    to_json.push(quote! {
                        Self::#item_ident(obj) => <#object_ty as #crate_name::types::ToJSON>::to_json(obj)
                    });
                }

                if variant.mapping.is_some() {
                    mapping.push(quote! {
                        (#mapping_name, format!("#/components/schemas/{}", <#object_ty as #crate_name::types::Type>::schema_ref().unwrap_reference()))
                    });
                }

                if let Some(discriminator_name) = &args.discriminator_name {
                    schemas.push(quote! {
                        #crate_name::registry::MetaSchemaRef::Inline(::std::boxed::Box::new(#crate_name::registry::MetaSchema {
                            required: #required,
                            all_of: ::std::vec![
                                <#object_ty as #crate_name::types::Type>::schema_ref(),
                                #crate_name::registry::MetaSchemaRef::Inline(::std::boxed::Box::new(#crate_name::registry::MetaSchema {
                                    properties: ::std::vec![
                                        (
                                            #discriminator_name,
                                            #crate_name::registry::MetaSchemaRef::merge(
                                                <::std::string::String as #crate_name::types::Type>::schema_ref(),
                                                #crate_name::registry::MetaSchema {
                                                    example: ::std::option::Option::Some(::std::convert::Into::into(#mapping_name)),
                                                    ..#crate_name::registry::MetaSchema::ANY
                                                }
                                            )
                                        )
                                    ],
                                    ..#crate_name::registry::MetaSchema::new("object")
                                }))
                            ],
                            ..#crate_name::registry::MetaSchema::ANY
                        }))
                    });
                } else {
                    schemas.push(quote! {
                        <#object_ty as #crate_name::types::Type>::schema_ref()
                    });
                }
            }
            _ => {
                return Err(
                    Error::new_spanned(&variant.ident, "Incorrect oneof definition.").into(),
                )
            }
        }
    }

    let discriminator = match &args.discriminator_name {
        Some(discriminator_name) => quote! {
            ::std::option::Option::Some(#crate_name::registry::MetaDiscriminatorObject {
                property_name: #discriminator_name,
                mapping: ::std::vec![#(#mapping),*],
            })
        },
        None => quote!(::std::option::Option::None),
    };

    let parse_from_json = match &args.discriminator_name {
        Some(discriminator_name) => quote! {
            let discriminator_name = value.as_object().and_then(|obj| obj.get(#discriminator_name));
            #(#from_json)*
            ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value))
        },
        None => quote! {
            #(#from_json)*
            ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value))
        },
    };

    let external_docs = match &args.external_docs {
        Some(external_docs) => {
            let s = external_docs.to_token_stream(&crate_name);
            quote!(::std::option::Option::Some(#s))
        }
        None => quote!(::std::option::Option::None),
    };

    let expanded = quote! {
        impl #crate_name::types::Type for #ident {
            const IS_REQUIRED: bool = true;

            type RawValueType = Self;

            type RawElementValueType = Self;

            fn name() -> ::std::borrow::Cow<'static, str> {
                ::std::convert::Into::into("object")
            }

            fn schema_ref() -> #crate_name::registry::MetaSchemaRef {
                #crate_name::registry::MetaSchemaRef::Inline(Box::new(#crate_name::registry::MetaSchema {
                    ty: "object",
                    title: #title,
                    description: #description,
                    external_docs: #external_docs,
                    any_of: ::std::vec![#(#schemas),*],
                    discriminator: #discriminator,
                    ..#crate_name::registry::MetaSchema::ANY
                }))
            }

            fn register(registry: &mut #crate_name::registry::Registry) {
                #(<#types as #crate_name::types::Type>::register(registry);)*
            }

            fn as_raw_value(&self) -> ::std::option::Option<&Self::RawValueType> {
                ::std::option::Option::Some(self)
            }

            fn raw_element_iter<'a>(&'a self) -> ::std::boxed::Box<dyn ::std::iter::Iterator<Item = &'a Self::RawElementValueType> + 'a> {
                ::std::boxed::Box::new(::std::iter::IntoIterator::into_iter(self.as_raw_value()))
            }
        }

        impl #crate_name::types::ParseFromJSON for #ident {
            fn parse_from_json(value: #crate_name::__private::serde_json::Value) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
                #parse_from_json
            }
        }

        impl #crate_name::types::ToJSON for #ident {
            fn to_json(&self) -> #crate_name::__private::serde_json::Value {
                match self {
                    #(#to_json),*
                }
            }
        }
    };

    Ok(expanded)
}
