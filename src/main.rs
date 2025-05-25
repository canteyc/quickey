use quickey::node::Node;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    let dict_csv = PathBuf::from("assets/words_with_usage.csv");
    let root = Node::load_dictionary_from_csv(&dict_csv);
    let after_csv = SystemTime::now();
    println!(
        "Loading from csv took {} seconds. Root has {} usages",
        after_csv.duration_since(start).unwrap().as_secs_f64(),
        root.usages(),
    );
}
