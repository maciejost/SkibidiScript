#[macro_use]
extern crate lazy_static;

mod keywords;

fn main() {
    // Access the static keywords
    for (js_kw, skibidi_kw) in keywords::KEYWORDS.iter() {
        println!("{:?} matches {:?}", js_kw, skibidi_kw);
    }
}
