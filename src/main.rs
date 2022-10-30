mod helpers;
mod loading;
mod ops;
mod params;

use std::collections::HashMap;
use std::env;

use crate::ops::*;

fn main() {
    let mut symbols: HashMap<String, String> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut ret_i: Vec<usize> = vec![usize::MAX - 1];
    let mut code: Vec<String> = Vec::new();
    let mut i: usize = 0;

    let args: Vec<String> = env::args().collect();

    loading::load(args[1].clone(), &mut code);
    loading::find_labels(&code, &mut labels);

    let mut ops: HashMap<
        String,
        &dyn Fn(
            &mut usize,
            &[String],
            &mut HashMap<String, String>,
            &mut HashMap<String, usize>,
            &mut Vec<usize>,
        ),
    > = HashMap::new();

    // ugly but tuples werent working
    ops.insert(s!("label"), &label);
    ops.insert(s!("sym"), &sym);
    ops.insert(s!("jmp"), &jmp);
    ops.insert(s!("je"), &je);
    ops.insert(s!("jne"), &jne);
    ops.insert(s!("jlt"), &jlt);
    ops.insert(s!("jgt"), &jgt);
    ops.insert(s!("jle"), &jle);
    ops.insert(s!("jge"), &jge);
    ops.insert(s!("print"), &print);
    ops.insert(s!("exit"), &exit);
    ops.insert(s!("ret"), &ret);

    ops.insert(s!("inc"), &inc);
    ops.insert(s!("dec"), &dec);

    while i < code.len() {
        ops[&code[i]](&mut i, &code, &mut symbols, &mut labels, &mut ret_i);
        i += 1;
    }
}
