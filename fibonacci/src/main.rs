use std::io;

fn fibonacci(num: usize) {
    let mut vec: Vec<i16> = vec![0; num];
    vec[1] = 1;
    for i in 2..vec.len() {
        vec[i] = vec[i - 2] + vec[i - 1];
    }
    for i in vec {
        print!("{i} ");
    }
    println!();
}

fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(word) => word,
        Err(e) => panic!("Bleh {}", e),
    };

    let num = match input.trim().parse() {
        Ok(n) => n,
        Err(e) => panic!("Not a number!\n{}", e),
    };

    println!("{}", num);

    fibonacci(num);
}
