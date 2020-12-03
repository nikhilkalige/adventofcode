use proc_macro::{TokenStream};
use syn::{parse_macro_input, Token};

use quote::quote;


#[derive(Debug)]
struct AocArgs {
    pub year: u32,
    pub start: u32,
    pub end: u32,
}

impl syn::parse::Parse for AocArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args: syn::punctuated::Punctuated<syn::LitInt, Token![,]> =
            input.parse_terminated(syn::parse::Parse::parse)?;

        let literals: Vec<u32> = args
            .iter()
            .map(|arg| arg.base10_parse::<u32>())
            .collect::<Result<_, _>>()?;

        match &(literals)[..] {
            &[year, start, end] => Ok(AocArgs { year, start, end }),
            _ => Err(input.error("Expected (year, start, end)")),
        }
    }
}


#[proc_macro]
pub fn aoc_run(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as AocArgs);
    println!("{:?}", args);

    let AocArgs {year, start ,end} = args;

    let mut runners = vec![];
    for day in start..=end {
        let year_ident = quote::format_ident!("aoc{}", year);
        let day_ident = quote::format_ident!("day{:02}", day);

        let parse = quote::format_ident!("parse");
        let part1 = quote::format_ident!("part1");
        let part2 = quote::format_ident!("part2");

        let input_location =  format!("/src/aoc{}/input/day{:02}", year, day);

        runners.push(quote! {
            let contents = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), #input_location));

            let input = #year_ident::#day_ident::#parse(&contents);

            println!("Day {}:", #day);

            let result = #year_ident::#day_ident::#part1(&input);
            println!("  Part 1: {}", result);

            let result = #year_ident::#day_ident::#part2(&input);
            println!("  Part 2: {}", result);

            println!("");
        });
    }

    let expanded = quote! {
        println!("Advent of Code {}", #year);
        println!("-------------------");
        #(#runners)*
    };

    // eprintln!("INPUT: {:#?}", expanded);

    TokenStream::from(expanded)
}
