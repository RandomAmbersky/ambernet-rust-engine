fn factorial(a: u16) -> f64 {
    let mut n = a as f64;
    let mut result = 1.0;
    while n > 0.0 {
        result = result * n;
        n = n - 1.0;
    }
    println!("{0:?}", result);
    return result;
}

#[no_mangle]
pub fn calculate_probability(num: u16, all: u16, j: u16) -> f32 {
    let total = factorial(all) / (factorial(num) * factorial(all-num));
    let mut acc = 0.0;
    let mut i = 1;
    while i <= j && i <= num {
        let a = factorial(j) / (factorial(i) * factorial(j - i));
        let b = factorial(all - j) / (factorial(num - i) * factorial(all - j + i - num));
        acc = acc + (a * b);
        i = i + 1;
    }
    ((acc / total) * 100.0) as f32
}

fn main () {
    println!("{0:?}", calculate_probability(5, 170, 1));
}
