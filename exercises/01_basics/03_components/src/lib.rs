// Generates Rust bindings for the WIT world at compile time.
wit_bindgen::generate!({
    path: "wit",
    world: "example",
    generate_all,
});

struct Example;

impl Guest for Example {
    fn add_random(num: u64) -> u64 {
        num.saturating_add(wasi::random::random::get_random_u64())
    }
}

export!(Example);
