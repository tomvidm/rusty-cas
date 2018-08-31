mod symexpr_rc;
mod symengine;
mod numeric;
mod lexer;
mod cli;

fn main() {
    let a = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let b = &a[2..5];
    println!("{}", a.len());
    println!("{}", &b.len());

    for (i, x) in b.iter().enumerate() {
        println!("{}", i);
    }
    cli::run();
}
