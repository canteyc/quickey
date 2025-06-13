use quickey::keyboard::qwerty;
use quickey::node::Node;
use quickey::predict::multi_predict;
use std::path::PathBuf;
use std::process::exit;
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
    let mut points = vec![];
    let term = console::Term::stdout();
    while let Ok(k) = term.read_key() {
        match k {
            console::Key::Unknown => todo!(),
            console::Key::UnknownEscSeq(_) => todo!(),
            console::Key::ArrowLeft => todo!(),
            console::Key::ArrowRight => todo!(),
            console::Key::ArrowUp => todo!(),
            console::Key::ArrowDown => todo!(),
            console::Key::Enter => todo!(),
            console::Key::Escape => todo!(),
            console::Key::Backspace => {
                points.pop();
            }
            console::Key::Home => todo!(),
            console::Key::End => todo!(),
            console::Key::Tab => todo!(),
            console::Key::BackTab => todo!(),
            console::Key::Alt => todo!(),
            console::Key::Del => todo!(),
            console::Key::Shift => todo!(),
            console::Key::Insert => todo!(),
            console::Key::PageUp => todo!(),
            console::Key::PageDown => todo!(),
            console::Key::Char(c) => {
                if c.eq(&' ') {
                    points.clear();
                } else {
                    points.push(
                        qwerty::LAYOUT
                            .iter()
                            .find(|key| key.ch == c as u8)
                            .unwrap()
                            .loc,
                    );
                }
            }
            console::Key::CtrlC => exit(0),
            _ => exit(1),
        }

        let hints = multi_predict(&root, &points);
        println!("{}", hints.join("\t"));
    }
}
