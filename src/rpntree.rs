use std::collections::HashMap;

use tree::Tree;

fn reducer_string(opt_left: Option<String>, center: char, opt_right: Option<String>) -> String {
    format!(
        "{}{}{}",
        opt_left.unwrap_or_else(|| { "".to_string() }),
        opt_right.unwrap_or_else(|| { "".to_string() }),
        center,
    )
}

pub fn test() {
    println!("adding characters to the binary tree");
    // let PRIO_MAP: HashMap<char, usize> = HashMap::new();
    let mut tree = Tree::new('+');
    let _ = &tree.root_node.insert_left('a');
    let _ = &tree.root_node.insert_right('b');
    println!("{}", &tree);
    let result = tree.reduce::<String>(reducer_string);
    println!("{}", result);
}
pub fn eval_infix(infixstr: &str) -> Result<String, String> {
    let mut priority_map = HashMap::<char, usize>::new();
    priority_map.insert('+', 100);
    priority_map.insert('-', 100);
    priority_map.insert('*', 200);
    priority_map.insert('/', 200);
    println!("Priority map: {:?}", priority_map);
    let waiting_stack: Vec<char> = vec![];
    for c in infixstr.chars() {
        if priority_map.contains_key(&c) {
            let priority = match priority_map.get(&c) {
                Some(c_ptr) => c_ptr.to_owned(),
                None => return Err("Internal error".to_string()),
            };
        }
    }
    Ok("Hello".to_string())
}
