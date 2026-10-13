// Generates Rust bindings for the WIT world at compile time.
wit_bindgen::generate!({
    path: "wit",
    world: "example",
    generate_all,
});

// This component should implement the interface laid out in the WIT file.
// If you get stuck, ask a neighbor or grab a trainer!
