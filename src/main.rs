//use lava_torrent::bencode::BencodeElem;

//use proc_macro::TokenStream;
//use quote::quote;
//use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_number::BencodeNumber;

mod variables;

fn main() {
    let mut a = BencodeArray::new();
    a.add(BencodeNumber::from(100));
    println!("{}", a.l.len());

    //println!("{}", a.l.get(0));

    //BencodeElem::Dictionary();
    //BencodeElem::String("asdasd".to_string());
    //let mut b = BencodeArray::new();
    //b.add(&BencodeArray::new());

    //println!("{}", b.l.len());
}

/*
#[derive(Debug)]
enum Bencode {
    Integer(i64),
    String(String),
    List(Vec<Bencode>),
    Dictionary(Vec<(String, Bencode)>),
}

fn encode_bencode(bencode: &Bencode) -> String {
    match bencode {
        Bencode::Integer(i) => format!("i{}e", i),
        Bencode::String(s) => format!("{}:{}", s.len(), s),
        Bencode::List(list) => {
            let encoded_items: Vec<String> = list.iter().map(encode_bencode).collect();
            format!("l{}e", encoded_items.join(""))
        }
        Bencode::Dictionary(dict) => {
            let mut encoded_items: Vec<String> = Vec::new();
            for (key, value) in dict {
                encoded_items.push(format!("{}{}", encode_bencode(&Bencode::String(key.clone())), encode_bencode(value)));
            }
            format!("d{}e", encoded_items.join(""))
        }
    }
}

fn main() {
    let data = Bencode::Dictionary(vec![
        ("name".to_string(), Bencode::String("Alice".to_string())),
        ("age".to_string(), Bencode::Integer(30)),
        ("hobbies".to_string(), Bencode::List(vec![
            Bencode::String("Programming".to_string()),
            Bencode::String("Gaming".to_string()),
        ])),
    ]);

    println!("{:?}", data);

    let encoded_data = encode_bencode(&data);
    println!("{}", encoded_data);
}
*/



/*



#[proc_macro_attribute]
pub fn my_annotation(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let args = parse_macro_input!(args as AttributeArgs);

    // Process the attribute arguments
    let mut my_attribute_value = None;
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("my_attribute") => {
                if let Lit::Str(value) = nv.lit {
                    my_attribute_value = Some(value.value());
                }
            }
            _ => {
                // Handle other attribute formats or ignore unrecognized attributes
            }
        }
    }

    // Parse the input tokens into a syntax tree
    let input_fn = parse_macro_input!(input as ItemFn);

    // Get the function name
    let fn_name = &input_fn.sig.ident;

    // Generate the modified function definition
    let expanded = quote! {
        #input_fn

        #[no_mangle]
        pub extern "C" fn #fn_name() {
            println!("Before function call: {}", stringify!(#fn_name));
            println!("Custom attribute value: {:?}", #my_attribute_value);
            #fn_name();
            println!("After function call: {}", stringify!(#fn_name));
        }
    };

    // Convert the generated tokens back into a TokenStream
    TokenStream::from(expanded)
}
*/

