use std::collections::HashMap;

pub enum RpnOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
fn apply_op(op: RpnOperation, v1: f32, v2: f32) -> f32 {
    match op {
        RpnOperation::Add => v1 + v2,
        RpnOperation::Subtract => v1 - v2,
        RpnOperation::Multiply => v1 * v2,
        RpnOperation::Divide => v1 / v2,
    }
}
fn rpn_match_op(c: char) -> Option<RpnOperation> {
    match c {
        '+' => Some(RpnOperation::Add),
        '-' => Some(RpnOperation::Subtract),
        '/' => Some(RpnOperation::Divide),
        '*' => Some(RpnOperation::Multiply),
        _ => None,
    }
}
pub fn eval(rpn_str: String, var_map: HashMap<char, f32>) -> Result<f32, ()> {
    let mut stack: Vec<f32> = vec![];
    for (_i, ch) in rpn_str.chars().enumerate() {
        println!("Parsing: {}, Stack: {:?}", ch, stack);
        let res = rpn_match_op(ch);
        if res.is_none() {
            let num = var_map.get(&ch);
            if num.is_none() {
                return Err(());
            }
            stack.push(*num.unwrap());
            continue;
        }
        let v2 = stack.pop();
        let v1 = stack.pop();
        if v1.is_none() || v2.is_none() {
            return Err(());
        }
        let num = apply_op(res.unwrap(), v1.unwrap(), v2.unwrap());
        println!("Result tmp: {}", num);
        stack.push(num)
    }
    if stack.len() != 1 {
        return Err(());
    }
    Ok(stack
        .pop()
        .expect("Expected there to be at least 1 element left..."))
}
