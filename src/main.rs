use quickey::node::Node;
use quickey::predict::multi_predict;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    colog::init();
    let start = SystemTime::now();
    let dict_csv = PathBuf::from("assets/words_alpha.txt");
    let root = Node::load_dictionary_from_txt(&dict_csv);
    let after_csv = SystemTime::now();
    println!(
        "Loading from csv took {} seconds. Root has {} usages",
        after_csv.duration_since(start).unwrap().as_secs_f64(),
        root.usages(),
    );
    let points = [(0, 0)];
    let hints = multi_predict(&root, &points);
    println!("{}", hints.join("\n"));
}
