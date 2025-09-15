use proc_macro::TokenStream;
use std::collections::HashMap;
use std::path::Path;
use crate::base::file_system::remove_last;

mod base;

#[proc_macro]
pub fn init_name_list(input: TokenStream) -> TokenStream {
    let expr:std::path::PathBuf = input.to_string().parse().unwrap();
    let src = expr.as_path();
    let mut name_list:HashMap<String,u32> = HashMap::new();
    let mut re = String::from("vec![");
    base::file_system::ls_dir(src, &mut |str|{
        let temp = name_list.entry(remove_last(str.display().to_string())).or_insert(0);
        *temp += 1;
    });
    for index in name_list.iter() {
        // println!("(String::from(\"{}\"),{}),",index.0,index.1);
        re += &format!("(String::from(\"{}\"),{}),",index.0,index.1);
    }
    re += "]";
    println!("{}",re);
    // re.parse().unwrap()
    TokenStream::new()
}