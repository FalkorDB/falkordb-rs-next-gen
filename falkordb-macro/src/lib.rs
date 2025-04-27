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
        let stream = match binary_op_alts.as_slice() {
            [one_alt] => generate_token_stream_one_alt(parse_exp, one_alt),
            [first_alt, second_alt] => {
                generate_token_stream_two_alt(parse_exp, first_alt, second_alt)
            }
            alts => generate_token_stream_many_alt(parse_exp, alts),
        };
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

fn generate_token_stream_one_alt(
    parse_exp: &Expr,
    one_alt: &BinaryOpAlt,
) -> proc_macro2::TokenStream {
    let token_match = &one_alt.token_match;
    let ast_constructor = &one_alt.ast_constructor;
    quote::quote! {
        let mut vec = Vec::new();
        loop {
            vec.push(#parse_exp);
            if self.lexer.current() == Token::#token_match {
                self.lexer.next();
            } else {
                if vec.len() == 1 {
                    return Ok(vec.pop().unwrap());
                }
                return Ok(QueryExprIR::#ast_constructor(vec));
            }
        }
    }
}

fn generate_token_stream_two_alt(
    parse_exp: &Expr,
    first_alt: &BinaryOpAlt,
    second_alt: &BinaryOpAlt,
) -> proc_macro2::TokenStream {
    let first_token_match = &first_alt.token_match;
    let first_ast_constructor = &first_alt.ast_constructor;
    let second_token_match = &second_alt.token_match;
    let second_ast_constructor = &second_alt.ast_constructor;

    quote::quote! {
        let mut vec = Vec::new();
        loop {
            loop {
                vec.push(#parse_exp);
                match self.lexer.current() {
                    Token::#first_token_match => {
                        self.lexer.next();
                    }
                    Token::#second_token_match => {
                        self.lexer.next();
                        break;
                    }
                    _ => {
                       if vec.len() == 1 {
                            return Ok(vec.pop().unwrap());
                       }
                       return Ok(QueryExprIR::#first_ast_constructor(vec));
                    }
                }
            }
            if 1 < vec.len(){
                vec = vec![QueryExprIR::#first_ast_constructor(vec)];
            }

            loop {
                vec.push(#parse_exp);
                match self.lexer.current() {
                    Token::#second_token_match => {
                        self.lexer.next();
                    }
                    Token::#first_token_match => {
                        self.lexer.next();
                        break;
                    }
                    _ => {
                       if vec.len() == 1 {
                            return Ok(vec.pop().unwrap());
                       }
                       return Ok(QueryExprIR::#second_ast_constructor(vec));
                    }
                }
            }

            if 1 < vec.len(){
                vec = vec![QueryExprIR::#second_ast_constructor(vec)];
            }
        }
    }
}

fn generate_token_stream_many_alt(
    parse_exp: &Expr,
    alts: &[BinaryOpAlt],
) -> proc_macro2::TokenStream {
    let alt_match = alts.iter().enumerate().map(|(i, alt)| {
        let token_match = &alt.token_match;
        let ast_constructor = &alt.ast_constructor;
        let match_legs = alts.iter().enumerate().map(|(j, alt_leg)| {
            let token_match = &alt_leg.token_match;
            if i == j {
                quote::quote! {
                    Token::#token_match => {
                       self.lexer.next();
                    }
                }
            } else {
                quote::quote! {
                     Token::#token_match => {
                        self.lexer.next();
                        current_op = Some(Token::#token_match);
                        break;
                    }
                }
            }
        });
        quote! {
            if current_op.is_none() || current_op == Some(Token::#token_match){
                loop {
                    vec.push(#parse_exp);
                    match self.lexer.current() {
                        #(#match_legs)*
                        _ => {
                           if vec.len() == 1 {
                                return Ok(vec.pop().unwrap());
                           }
                           return Ok(QueryExprIR::#ast_constructor(vec));
                        }
                    }
                }
                if 1 < vec.len(){
                    vec = vec![QueryExprIR::#ast_constructor(vec)];
                }
            }
        }
    });

    quote::quote! {
        let mut vec = Vec::new();
        let mut current_op: Option<Token> = None;
        loop {
            #(#alt_match)*
        }
    }
}
