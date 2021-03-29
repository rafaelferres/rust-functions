fn dobro(num: i32) -> i32 {
    num * 2
}

fn maior(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

fn albuma_fn(par_a: f32, par_b: i128) -> f32 {
    println!("Essa função devolve um valor flutuante");
    10.1 * par_a + par_b as f32
}

fn main() {
    println!("O dobro de {} é {}", 5, dobro(5));
    println!("O maior numero entre {} e {} é {}", 5, 10, maior(5, 10));
    println!("{}", albuma_fn(5.1, 10));
}
