mod rpn;
use std::{collections::HashMap, io::stdin};
/*struct TextSettings {}
struct Component {
    text: String,
    setting: TextSettings,
}
struct Article {
    title: String,
    components: Vec<Component>,
}
impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
impl Component {
    fn new(text: String, setting: Option<TextSettings>) -> Component {
        match setting {
            Some(s) => {
                return Component {
                    text,
                    setting: s,
                }
            },
            None => {
                return Component {
                    text,
                    setting: TextSettings {}
                }
            }
        }
    }
}
impl Article {
    fn new(lines: Vec<String>) -> Article {
        let mut components = vec![];
        for (i, l) in lines.iter().enumerate() {
            components.insert(i, Component::new(l.to_string(), None));
        }
        Article {
            title: "article title".to_string(),
            components,
        }
    }
}
impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = String::from("");
        for c in &self.components {
            builder += &format!("{}\n", c);
        }
        write!(f, "{}", builder)
    }
}*/
fn main() {
    let mut vars: HashMap<char, f32> = HashMap::new();
    println!("Input your variable mappings below, in K:char,V:f32 format, * to break;");
    let term = stdin();
    loop {
        let mut s = String::new();
        let _ = term.read_line(&mut s);
        if s.trim() == "*" {
            break;
        }
        let vals = s.trim().split_once(',');
        if vals.is_none() {
            println!("Format invalid!");
            return;
        }
        let (k_s, v_s) = vals.unwrap();
        println!("Stored: '{}': '{}'", k_s, v_s);
        if k_s.chars().count() != 1 {
            println!("Identifier must only be one character!");
            return;
        }
        let k = k_s.chars().next();
        if k.is_none() {
            return;
        }
        vars.insert(k.unwrap(), v_s.parse::<f32>().unwrap());
    }
    println!("Great! Now put your RPN expression below: ");
    let mut rpn_str = String::new();
    let _ = term.read_line(&mut rpn_str);
    let result = rpn::eval(rpn_str.trim().to_string(), vars);
    match result {
        Ok(number) => println!("Result: {}", number),
        Err(_) => println!("Error occurred"),
    }
}
/*fn _main() -> Result<(), Box<dyn Error>> {
    println!("running main function");
    let mut cont = true;
    let mut lines: Vec<String> = vec![];
    while true {
        let mut s = String::new();
        let input = stdin();
        let _size = input.read_line(&mut s);
        println!("{}", s);
        if s.eq("stop") { break; }
        lines.push(s);
    }
    let a = Article::new(lines);
    println!("{}", a);
    Ok(())
}*/
