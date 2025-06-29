use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, ExprLit, ItemFn};

#[proc_macro_attribute]
pub fn time_run(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let challenge_number = parse_macro_input!(attr as ExprLit);

    let sig = input.sig;
    let block = input.block;

    let expanded = quote!(
        #sig {
            let mut times: [std::time::Duration; 1000] = [std::time::Duration::default(); 1000];

            let mut result: String = String::default();

            for i in 0..1000 {
                let start_time = std::time::Instant::now();
                result = #block;
                let duration = start_time.elapsed();
                times[i] = duration;
            }
            let min = times.iter().min().unwrap();
            let max = times.iter().max().unwrap();
            let total = times.into_iter().fold(std::time::Duration::from_secs(0), |acc, d| acc + d);
            let average = total / 1000;

            println!("Answer: {}", result);
            println!("Time taken for challenge: {}: \n min: {:?} \n max: {:?} \n average time: {:?}", #challenge_number, min, max, average)
        }
    );

    expanded.into()
}

#[proc_macro_attribute]
pub fn time_run2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let challenge_number = parse_macro_input!(attr as ExprLit);

    let sig = input.sig;
    let block = input.block;

    let expanded = quote!(
        #sig {
            let mut times: [std::time::Duration; 1000] = [std::time::Duration::default(); 1000];

            let mut result1: String = String::default();
            let mut result2: String = String::default();

            for i in 0..1 {
                let start_time = std::time::Instant::now();
                (result1, result2) = #block;
                let duration = start_time.elapsed();
                times[i] = duration;
            }
            let min = times.iter().min().unwrap();
            let max = times.iter().max().unwrap();
            let total = times.into_iter().fold(std::time::Duration::from_secs(0), |acc, d| acc + d);
            let average = total / 1000;

            println!("Answer: \n part1: {}\n part2: {}", result1, result2);
            println!("Time taken for challenge: {}: \n min: {:?} \n max: {:?} \n average time: {:?}", #challenge_number, min, max, average)
        }
    );

    expanded.into()
}
