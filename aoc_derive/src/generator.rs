use proc_macro2 as pm2;
use quote::{quote, TokenStreamExt};
use std::convert::TryFrom;
use syn;
use syn::Token;

use crate::types::{Day, DayPart, ParserResultType, Part, Solution};

#[derive(Debug)]
pub struct ModuleArgs {
    pub year: u32,
    pub days: (Day, Day),
}

#[derive(Debug)]
pub struct SolverArgs {
    pub day: Day,
    pub part: Part,
    // pub name: Option<&str>,
}

impl syn::parse::Parse for ModuleArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year_token: syn::LitInt = input.parse()?;
        let _: Token![,] = input.parse()?;
        let start = input.parse()?;
        let _: Token![,] = input.parse()?;
        let end = input.parse()?;

        let year = year_token.base10_parse::<u32>()?;

        Ok(ModuleArgs {
            year,
            days: (start, end),
        })
    }
}

impl syn::parse::Parse for Day {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let day = ident.to_string().parse::<Day>();

        day.map_err(|err| input.error(err))
    }
}

impl syn::parse::Parse for Part {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let part = ident.to_string().parse::<Part>();

        part.map_err(|err| input.error(err))
    }
}

impl syn::parse::Parse for SolverArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let day = input.parse()?;
        let _: Token![,] = input.parse()?;
        let part = input.parse()?;

        Ok(SolverArgs { day, part })
    }
}

impl quote::ToTokens for Day {
    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        // let name = format!("day{:02}", *self as u8);
        let name = format!("{:?}", self);
        tokens.append(syn::Ident::new(&name, pm2::Span::call_site()));
    }
}

impl quote::ToTokens for Part {
    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let name = format!("{:?}", self);
        tokens.append(syn::Ident::new(&name, pm2::Span::call_site()));
    }
}

pub fn day_imports(start: u32, end: u32) -> syn::Result<pm2::TokenStream> {
    let days: Result<Vec<Day>, &str> = (start..=end).map(|day| Day::try_from(day)).collect();

    match days {
        Ok(days) => {
            let imports: pm2::TokenStream = days
                .iter()
                .map(|day| {
                    // Temporarily disable to get rid of rust analyzer errors
                    let _day_ident = syn::Ident::new(
                        &day.to_string().to_ascii_lowercase(),
                        pm2::Span::call_site(),
                    );
                    quote! {
                        // pub mod #day_ident;
                    }
                })
                .collect();

            Ok(imports.into())
        }
        Err(error) => Err(syn::Error::new(pm2::Span::call_site(), error)),
    }
}

pub fn default_implementations(start: u32, end: u32) -> pm2::TokenStream {
    let parser_impls: pm2::TokenStream = (start..=end)
        .map(|day| {
            let day = Day::try_from(day).unwrap();
            let struct_name = syn::Ident::new(&format!("{}Parser", day), pm2::Span::call_site());

            quote! {
                pub struct #struct_name;
                impl ParserDefault for #struct_name {}
            }
        })
        .collect();

    let solver_impls: pm2::TokenStream = day_parts(start, end)
        .map(|day_part| {
            let solver_name = camelcase_identifier(&day_part, "Solver", true);

            quote! {
                pub struct #solver_name<I>(pub PhantomData<I>);
                impl<I> SolverDefault for #solver_name<I> {
                    type Input = I;
                }
            }
        })
        .collect();

    quote! {
        pub mod __aoc {
            use aoc::{Solver, Parser, ParserDefault, SolverDefault};
            use std::marker::PhantomData;

            #parser_impls
            #solver_impls
        }
    }
}

pub fn parser(day: Day, function: syn::ItemFn) -> syn::Result<pm2::TokenStream> {
    let output = if let syn::ReturnType::Type(_, r) = &function.sig.output {
        r.clone()
    } else {
        let msg = format!(
            "Parser function ({}) should have an output",
            function.sig.ident
        );
        return Err(syn::Error::new(pm2::Span::call_site(), msg));
    };

    let (wrapper, output) = if let Some((ty, inner)) = extract_result_type(&output) {
        (Some(ty), Box::new(inner))
    } else {
        (None, output)
    };



    let name = &function.sig.ident;

    let parser_body = match wrapper {
        Some(ParserResultType::Result) => {
            quote! { #name(input).map_err(|err| err.into()) }
        }
        Some(ParserResultType::Option) => {
            quote! { #name(input).ok_or_else(|| aoc::ParserFailed.into()) }
        }
        None => quote! { Ok(#name(input)) },
    };

    let struct_name = syn::Ident::new(&format!("{}Parser", day), pm2::Span::call_site());

    // #[doc(hidden)]
    // pub mod
    let implementation = quote! {
        use super::__aoc::#struct_name;
        #function

        impl<'a> aoc::Parser<'a> for #struct_name {
            type Output = #output;

            fn parse(&self, input: &'a str) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                #parser_body
            }
        }

    };

    Ok(implementation)
}

pub fn solver(day: Day, part: Part, solver: syn::ItemFn) -> syn::Result<pm2::TokenStream> {
    let dp = DayPart {
        day,
        part,
        solution: Solution::Solution1,
        name: None,
    };

    let input_type = match solver
        .sig
        .inputs
        .first()
        .expect("Solver must take an input")
    {
        syn::FnArg::Typed(pat) => pat.ty.clone(),
        _ => {
            return Err(syn::Error::new(
                pm2::Span::call_site(),
                "Solver function cannot have a self parameter",
            ))
        }
    };

    // let input_type = match &input_type {
    //     syn::Type::Reference(ty) => {
    //         let ty = ty.clone();
    //         let x = Box::new(ty)
    //         x
    //     },
    //     _ => input_type,
    // }

    let output_type = if let syn::ReturnType::Type(_, ref output_type) = solver.sig.output {
        output_type.clone()
    } else {
        return Err(syn::Error::new(
            pm2::Span::call_site(),
            "Solver function should return an output",
        ));
    };

    let (wrapper, output_type) = if let Some((ty, inner)) = extract_result_type(&output_type) {
        (Some(ty), Box::new(inner))
    } else {
        (None, output_type)
    };

    let name = &solver.sig.ident;

    let solver_body = match wrapper {
        Some(ParserResultType::Result) => {
            quote! { #name(input).map_err(|err| err.into()) }
        }
        Some(ParserResultType::Option) => {
            quote! { #name(input).ok_or_else(|| aoc::SolverFailed.into()) }
        }
        None => quote! { Ok(#name(input)) },
    };

    let struct_name = camelcase_identifier(&dp, "Solver", true);
    let code = quote! {
        use super::__aoc::#struct_name;

        #solver

        impl aoc::Solver<#input_type> for #struct_name<#input_type> {
            type Output = #output_type;

            fn solve(&self, input: #input_type) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                #solver_body
            }
        }
    };

    Ok(code)
}

pub fn main_runner(year: u32, start: u32, end: u32) -> syn::Result<pm2::TokenStream> {
    let code: pm2::TokenStream = (start..=end)
        .map(|day| {
            let day_e = Day::try_from(day).unwrap();
            let input_file = format!("/src/aoc{}/input/day{:02}", year, day);
            let parser_name = syn::Ident::new(&format!("{}Parser", day_e), pm2::Span::call_site());

            let parts: pm2::TokenStream = Part::variants()
                .iter()
                .map(|&part| {
                    let solutions: pm2::TokenStream = Solution::variants()
                        .iter()
                        .map(|&solution| solution_runner(day_e, part, solution))
                        .collect();

                    let part_num = part as u8;
                    quote! {
                        println!("\n  Part {:?}: ", #part_num);

                        #solutions
                    }
                })
                .collect();

            quote! {
                {
                    let contents = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), #input_file));
                    let parser = &#parser_name;

                    let mut timings: HashMap<_, _> = HashMap::<&str, Vec<(&str, Duration)>>::new();
                    let mut parser_time: Option<Duration> = None;

                    println!("Day {:02}:", #day);

                    let start_time = Instant::now();
                    match parser.parse(contents) {
                        Ok(parsed_input) => {
                            let end_time = Instant::now();
                            parser_time = Some(end_time - start_time);

                            #parts
                        },
                        Err(e) => eprintln!("\tParser failed: {:?}", e),
                    }

                    if let Some(t) = parser_time {
                        println!("\n  Parser: {:?}", t);
                        let mut parts: Vec<_> = timings.keys().map(|&k| k).collect();
                        parts.sort();
                        for part in parts {
                            println!("  {}: ", part);

                            let vec_entries = timings.entry(part).or_default();
                            for (sol, t) in vec_entries {
                                println!("\t{}: {:?}", sol, t);
                            }
                        }
                        println!("");
                    }
                }
            }
        })
        .collect();

    let module_name = quote::format_ident!("aoc{}", year);
    let main_code = quote! {
        {
            use #module_name::__aoc::*;
            use std::marker::PhantomData;
            use std::time::{Duration, Instant};
            use aoc::{Parser, Solver};
            use std::collections::HashMap;

            let year:u32 = #year;

            println!("Advent of Code {}\n", year);

            #code

        }
    };

    Ok(main_code)
}

fn solution_runner(day: Day, part: Part, solution: Solution) -> pm2::TokenStream {
    let solver_name = camelcase_identifier(
        &DayPart {
            day,
            part,
            solution,
            name: None,
        },
        "Solver",
        true,
    );

    let part_name = part.to_string();
    let solution_name = solution.to_string();

    quote! {
        let solver = &#solver_name(PhantomData);
        if solver.is_implemented() {
            let solver_start = Instant::now();
            match solver.solve(&parsed_input) {
                Ok(result) => {
                    let solver_end = Instant::now();
                    timings.entry(#part_name).or_default().push((#solution_name, solver_end - solver_start));
                    println!("\t  {} : {:?}", #solution_name, result);
                },
                Err(e) => eprintln!("\t failed {}", e),
            }
        }
    }
}

fn day_parts(start: u32, end: u32) -> impl Iterator<Item = DayPart<'static>> {
    (start..=end).into_iter().flat_map(move |day| {
        (1..=2).flat_map(move |part| {
            (1..=4).into_iter().map(move |sol| DayPart {
                day: Day::try_from(day).unwrap(),
                part: Part::try_from(part).unwrap(),
                solution: Solution::try_from(sol).unwrap(),
                name: None,
            })
        })
    })
}

fn camelcase_identifier(day_part: &DayPart, suffix: &str, use_solution: bool) -> syn::Ident {
    let DayPart {
        day,
        part,
        solution,
        ..
    } = day_part;

    let name = if use_solution {
        format!("{}{}{}{}", day, part, solution, suffix)
    } else {
        format!("{}{}{}", day, part, suffix)
    };

    syn::Ident::new(&name, pm2::Span::call_site())
}

fn extract_result_type(ty: &syn::Type) -> Option<(ParserResultType, syn::Type)> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments: s, .. },
        ..
    }) = ty
    {
        if let Some(p) = s.last() {
            if (p.ident == "Result") || (p.ident == "Option") {
                if let syn::PathArguments::AngleBracketed(a) = &p.arguments {
                    if let Some(arg) = a.args.first() {
                        if let syn::GenericArgument::Type(t) = arg {
                            let parser_type = if p.ident == "Result" {
                                ParserResultType::Result
                            } else {
                                ParserResultType::Option
                            };

                            return Some((parser_type, t.clone()));
                        }
                    }
                }
            }
        }
    }

    None
}
