use quote::quote;
use syn::parse::{Parse, ParseStream};
// Grammar
// binary_op: parse_exp, binary_op_alts
// binary_op_alts: binary_op_alt (, binary_op_alt)*
// binary_op_alt: token_match => ast_constructor

// examples:
// parse_binary_expr!(self.parse_power_expr()?, Star => Mul);
// parse_binary_expr!(self.parse_power_expr()?, Star => Mul, Slash => Div);
// parse_binary_expr!(self.parse_power_expr()?, Star => Mul, Slash => Div, Modulo => Modulo);

use syn::{Expr, Result};

#[proc_macro]
pub fn parse_binary_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let binary_op = syn::parse_macro_input!(input as BinaryOp);
    quote! { #binary_op }.into()
}

struct BinaryOp {
    parse_exp: Expr,
    binary_op_alts: Vec<BinaryOpAlt>,
}

impl Parse for BinaryOp {
    fn parse(input: ParseStream) -> Result<Self> {
        let parse_exp = input.parse()?;
        _ = input.parse::<syn::Token![,]>()?;
        let binary_op_alts =
            syn::punctuated::Punctuated::<BinaryOpAlt, syn::Token![,]>::parse_separated_nonempty(
                input,
            )?;
        Ok(BinaryOp {
            parse_exp,
            binary_op_alts: binary_op_alts.into_iter().collect(),
        })
    }
}

impl quote::ToTokens for BinaryOp {
    fn to_tokens(
        &self,
        tokens: &mut proc_macro2::TokenStream,
    ) {
        let binary_op_alts = &self.binary_op_alts;
        let parse_exp = &self.parse_exp;
        let stream = generate_token_stream(parse_exp, binary_op_alts);
        tokens.extend(stream);
    }
}

struct BinaryOpAlt {
    token_match: syn::Ident,
    ast_constructor: syn::Ident,
}

impl Parse for BinaryOpAlt {
    fn parse(input: ParseStream) -> Result<Self> {
        let token_match = input.parse()?;
        _ = input.parse::<syn::Token![=>]>()?;
        let ast_constructor = input.parse()?;
        Ok(BinaryOpAlt {
            token_match,
            ast_constructor,
        })
    }
}
fn generate_token_stream(
    parse_exp: &Expr,
    alts: &[BinaryOpAlt],
) -> proc_macro2::TokenStream {
    let whiles = alts.iter().map(|alt| {
        let token_match = &alt.token_match;
        let ast_constructor = &alt.ast_constructor;
        quote::quote! {
            while Token::#token_match == self.lexer.current() {
               self.lexer.next();
               vec.push(#parse_exp);
            }
            if vec.len() > 1 {
                vec = vec![QueryExprIR::#ast_constructor(vec)];
            }
        }
    });
    let tokens = alts.iter().map(|alt| {
        let token_match = &alt.token_match;
        quote::quote! {
            Token::#token_match
        }
    });

    quote::quote! {
        let mut vec = Vec::new();
        vec.push(#parse_exp);
        loop {
            #(#whiles)*
            if ![#(#tokens,)*].contains(&self.lexer.current()) {
                return Ok(vec.pop().unwrap());
            }
        }
    }
}
