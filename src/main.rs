use quickey::{math::frac, node::Node};
use std::time::SystemTime;
use std::{fs::write, path::PathBuf};

fn main() {
    let start = SystemTime::now();
    let dict_file = PathBuf::from("assets/words_alpha.txt");
    let root = Node::load_dictionary_from_txt(&dict_file);
    let after_txt = SystemTime::now();
    println!(
        "Loading from txt took {} seconds. Root has {} usages",
        after_txt.duration_since(start).unwrap().as_secs_f64(),
        root.usages(),
    );

    let mut dict_json = dict_file;
    dict_json.set_extension("json");
    let root = Node::load_dictionary_from_json(&dict_json);
    let after_json = SystemTime::now();
    println!(
        "Loading from json took {} seconds. Root has {} usages",
        after_json.duration_since(after_txt).unwrap().as_secs_f64(),
        root.usages(),
    );

    let dict_csv = PathBuf::from("assets/words_with_usage.csv");
    let root = Node::load_dictionary_from_csv(&dict_csv);
    let after_csv = SystemTime::now();
    println!(
        "Loading from csv took {} seconds. Root has {} usages",
        after_csv.duration_since(after_json).unwrap().as_secs_f64(),
        root.usages(),
    );
}
