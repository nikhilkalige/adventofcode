use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


///
/// ```
/// aoc_main!({
///     year 2019;
/// },
/// {
///     year 2020;
/// })
/// ```
#[proc_macro]
pub fn aoc_main(input: TokenStream) -> TokenStream {



    TokenStream::new()
}




// mod generator;
// mod types;

// #[proc_macro]
// pub fn aoc_main(input: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(input as generator::ModuleArgs);
//     let code = generator::main_runner(args.year, args.days.0 as u32, args.days.1 as u32);
//     if code.is_err() {
//         return code.unwrap_err().to_compile_error().into();
//     }

//     code.unwrap().into()
// }

// #[proc_macro]
// pub fn aoc_new_mod(input: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(input as generator::ModuleArgs);
//     let imports = generator::day_imports(args.days.0 as u32, args.days.1 as u32);
//     if imports.is_err() {
//         return imports.unwrap_err().to_compile_error().into();
//     }

//     let imports = imports.unwrap();
//     let implementations =
//         generator::default_implementations(args.days.0 as u32, args.days.1 as u32);
//     let module = quote! {
//         #imports
//         #implementations
//     };

//     module.into()
// }

// #[proc_macro_attribute]
// pub fn aoc_parser(args: TokenStream, input: TokenStream) -> TokenStream {
//     let day = parse_macro_input!(args as types::Day);
//     let parser = parse_macro_input!(input as ItemFn);

//     let implementation = generator::parser(day, parser);
//     if implementation.is_err() {
//         return implementation.unwrap_err().to_compile_error().into();
//     }

//     let code = implementation.unwrap();
//     (quote! { #code }).into()
// }

// #[proc_macro_attribute]
// pub fn aoc_solver(args: TokenStream, input: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(args as generator::SolverArgs);
//     let solver = parse_macro_input!(input as ItemFn);

//     let implementation = generator::solver(args.day, args.part, solver);
//     if implementation.is_err() {
//         return implementation.unwrap_err().to_compile_error().into();
//     }

//     let code = implementation.unwrap();
//     (quote! { #code }).into()
// }
