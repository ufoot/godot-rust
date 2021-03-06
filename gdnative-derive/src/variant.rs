use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Generics, Ident};

mod attr;
mod bounds;
mod from;
mod repr;
mod to;

use bounds::extend_bounds;
use repr::{Repr, VariantRepr};

pub(crate) struct DeriveData {
    pub(crate) ident: Ident,
    pub(crate) repr: Repr,
    pub(crate) generics: Generics,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Direction {
    To,
    From,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ToVariantTrait {
    ToVariant,
    OwnedToVariant,
}

impl ToVariantTrait {
    fn trait_path(self) -> syn::Path {
        match self {
            Self::ToVariant => parse_quote! { ::gdnative::core_types::ToVariant },
            Self::OwnedToVariant => parse_quote! { ::gdnative::core_types::OwnedToVariant },
        }
    }

    fn to_variant_fn(self) -> syn::Ident {
        match self {
            Self::ToVariant => parse_quote! { to_variant },
            Self::OwnedToVariant => parse_quote! { owned_to_variant },
        }
    }

    fn to_variant_receiver(self) -> syn::Receiver {
        match self {
            Self::ToVariant => parse_quote! { &self },
            Self::OwnedToVariant => parse_quote! { self },
        }
    }
}

pub(crate) fn parse_derive_input(
    input: TokenStream,
    bound: &syn::Path,
    dir: Direction,
) -> DeriveData {
    let input = match syn::parse_macro_input::parse::<DeriveInput>(input) {
        Ok(val) => val,
        Err(err) => {
            panic!("{}", err);
        }
    };

    let repr = match input.data {
        Data::Struct(struct_data) => Repr::Struct(VariantRepr::repr_for(&struct_data.fields)),
        Data::Enum(enum_data) => Repr::Enum(
            enum_data
                .variants
                .iter()
                .map(|variant| {
                    (
                        variant.ident.clone(),
                        VariantRepr::repr_for(&variant.fields),
                    )
                })
                .collect(),
        ),
        Data::Union(_) => panic!("Variant conversion derive macro does not work on unions."),
    };

    let generics = extend_bounds(input.generics, &repr, bound, dir);

    DeriveData {
        ident: input.ident,
        repr,
        generics,
    }
}

pub(crate) fn derive_to_variant(trait_kind: ToVariantTrait, input: TokenStream) -> TokenStream {
    to::expand_to_variant(
        trait_kind,
        parse_derive_input(input, &trait_kind.trait_path(), Direction::To),
    )
}

pub(crate) fn derive_from_variant(input: TokenStream) -> TokenStream {
    let bound: syn::Path = syn::parse2(quote! { ::gdnative::core_types::FromVariant }).unwrap();
    from::expand_from_variant(parse_derive_input(input, &bound, Direction::From))
}
