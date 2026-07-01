fn main() {
    let greeting = rdgnru_lib::greet("Rust");
    let slug = rdgnru_lib::slugify("Hello, Rust World!");

    println!("{greeting}");
    println!("slug: {slug}");
}
