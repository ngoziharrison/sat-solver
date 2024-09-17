// goal is is to implement the DPLL algorithm

use std::env;
use std::collections::HashMap;


fn unit_resolution(s: String) -> String {
    return s

}

fn encode_clause(s: &str, h: &HashMap<&char, i32>) -> Vec<i32>{
   s.split(" ")
       .map(|x| 
            match x {
                x if x.starts_with("~") => h[&x.chars().collect::<Vec<char>>()[1]] + 1,
                _ => h[&x.chars().collect::<Vec<char>>()[0]]

            })
        .collect()

}


fn main(){
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    
    let input = &args[1];
    let mut variables: Vec<char> = input
        .chars()
        .filter(|&x| x.is_alphanumeric() == true)
        .collect();
    variables.sort();
    variables.dedup();

    let n: i32 = variables.len() as i32;
    let variable_encoding : Vec<i32> = (0..n).collect(); 

    let mut variable_table = HashMap::new();

    for (iter, item) in variables.iter().enumerate() {
        variable_table.insert(item, variable_encoding[iter]);    

    }
    let encoded_clauses: Vec<Vec<i32>> = input.split(",")
        .map(|x| encode_clause(x.trim(), &variable_table))
        .collect();

    println!("list of variables {:?}", variables);
    println!("variable table {:?}", variable_table);
    println!("encoded clauses {:?}", encoded_clauses)

}
