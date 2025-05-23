use std::{fs::write, path::PathBuf};

use quickey::{
    math::frac,
    node::Node,
};

fn main() {
    let dict_file = PathBuf::from("assets/words_alpha.txt");
    let root = Node::load_dictionary_from_txt(&dict_file);
    for child in root.children() {
        println!(
            "{}: {:.3}",
            child.char(),
            frac(child.usages(), root.usages())
        );
    }
    let mut dict_json = dict_file;
    dict_json.set_extension("json");
    write(dict_json, serde_json::to_string_pretty(&root).unwrap()).unwrap();
}
