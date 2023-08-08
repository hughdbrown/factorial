
fn recursive_factorial(n: i64) -> i64 {
    if n <= 1 { 1i64 } else {n * recursive_factorial(n - 1)}
}

fn iterative_factorial(n: i64) -> i64 {
    let mut result = 1i64;
    for i in 2i64..=n {
        result *= i;
    }
    result
}

fn main() {
    println!("Recursive");
    for i in 0..21 {
        println!("{i}: factorial({i}) = {}", recursive_factorial(i as i64));
    }

    println!("Iterative");
    for i in 0..21 {
        println!("{i}: factorial({i}) = {}", iterative_factorial(i as i64));
    }
}
