use crate::util::{
    to_camel_case, AttributeArguments, QuotableBorrowedStr, QuotableDirection, QuotableOption,
};
use azure_functions_shared::codegen;
use proc_macro::Diagnostic;
use quote::ToTokens;
use std::borrow::Cow;
use std::convert::TryFrom;
use syn::spanned::Spanned;
use syn::Lit;

pub struct BlobTrigger<'a>(pub Cow<'a, codegen::bindings::BlobTrigger>);

impl TryFrom<AttributeArguments> for BlobTrigger<'_> {
    type Error = Diagnostic;

    fn try_from(args: AttributeArguments) -> Result<Self, Self::Error> {
        let mut name = None;
        let mut path = None;
        let mut connection = None;

        for (key, value) in args.list.iter() {
            let key_str = key.to_string();

            match key_str.as_str() {
                "name" => match value {
                    Lit::Str(s) => {
                        name = Some(Cow::Owned(to_camel_case(&s.value())));
                    }
                    _ => {
                        return Err(value
                            .span()
                            .unstable()
                            .error("expected a literal string value for the 'name' argument"));
                    }
                },
                "path" => match value {
                    Lit::Str(s) => {
                        path = Some(Cow::Owned(s.value()));
                    }
                    _ => {
                        return Err(value
                            .span()
                            .unstable()
                            .error("expected a literal string value for the 'path' argument"));
                    }
                },
                "connection" => match value {
                    Lit::Str(s) => {
                        connection = Some(Cow::Owned(s.value()));
                    }
                    _ => {
                        return Err(value.span().unstable().error(
                            "expected a literal string value for the 'connection' argument",
                        ));
                    }
                },
                _ => {
                    return Err(key.span().unstable().error(format!(
                        "unsupported binding attribute argument '{}'",
                        key_str
                    )));
                }
            };
        }

        if path.is_none() {
            return Err(args
                .span
                .error("the 'path' argument is required for blob trigger bindings."));
        }

        Ok(BlobTrigger(Cow::Owned(codegen::bindings::BlobTrigger {
            name: name.expect("expected a name for a blob trigger binding"),
            path: path.expect("expected a path for a blob trigger binding"),
            connection,
            direction: codegen::Direction::In,
        })))
    }
}

impl ToTokens for BlobTrigger<'_> {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let name = QuotableBorrowedStr(&self.0.name);
        let path = QuotableBorrowedStr(&self.0.path);
        let connection = QuotableOption(self.0.connection.as_ref().map(|x| QuotableBorrowedStr(x)));
        let direction = QuotableDirection(self.0.direction.clone());

        quote!(::azure_functions::codegen::bindings::BlobTrigger {
            name: #name,
            path: #path,
            connection: #connection,
            direction: #direction,
        })
        .to_tokens(tokens)
    }
}
