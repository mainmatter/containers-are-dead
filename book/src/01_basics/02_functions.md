# Functions

WebAssembly functions can only pass and return basic numeric types: `i32`, `i64`, `f32`, and `f64`. [^1]

## Function Exports in Rust

To export a Rust function so it can be called by the WebAssembly host, use `extern "C"`:

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

The `extern "C"` calling convention ensures the function uses C-style ABI, which allows other languages to call it correctly when importing the Wasm module. The `#[no_mangle]` attribute prevents Rust from changing the function name during compilation, making it accessible under its original name. This is important since all imports and exports in WebAssembly are referenced by name.

## Passing Complex Data

Since WebAssembly can only pass numbers directly, complex data like strings must be passed as pointers and lengths:

```rust
#[no_mangle]
pub extern "C" fn process_string(ptr: *const u8, len: usize) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let string = std::str::from_utf8(slice).unwrap();
    // Process the string...
    string.len() as i32
}
```

Callers write string data to WebAssembly linear memory and passes the memory address (`ptr`) and byte length (`len`) as function parameters.

## Exercise

Over the course of this workshop we will be building a "calculator-as-a-service" backend - compiled to WebAssembly - that will parse and evaluate arithmetic expressions of the form `42 * 8 / 16.2`. We will be building up this service in small steps throughout the exercises.

The exercise for this section requires you to build a parser that transforms an input `&str` into a sequence of `Tokens`. Our parser should skip over any whitespace characters it encounters and should be able to parse the following tokens:

```rust
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Numbers: 1, or 4.5, or 100000000. Must fit within an f64.
    Number(f64),
    /// The `+` symbol
    Plus,
    /// The `-` symbol
    Minus,
    /// The `*` symbol
    Multiply,
    /// The `/` divide symbol
    Divide,
    /// The `(` symbol
    LeftParen,
    /// The `)` symbol
    RightParen,
}
```

You can use the [`.chars()`] iterator to obtain an iterator over characters in a Rust string and use that to process the incoming string into tokens.

If you feel confident and adventurous feel free to come up with your own implementations, but each chapter will include a few code snippets at the end as a starting point or inspiration so you don't get stuck.

<details><summary><b>Hint 01</b></summary>

You can use the [`.peekable()`] iterator combinator and its [`.next_if()`] method to skip over whitespace characters in the input:

```rust
match ch {
    // skip over whitespace
    ' ' | '\t' => {
        while self.chars.next_if(|c| *c == ' ' || *c == '\t').is_some() {}
        return self.next();
    }
    // ... other cases
}
```

</details>

<details><summary><b>Hint 02</b></summary>

You can also use the [`.peekable()`] iterator combinator and its [`.next_if()`] method to collect all number tokens
into a string and then use the [`f64::from_str`] implementation to parse it into an `f64`.

```rust
match ch {
    // skip over whitespace
    ' ' | '\t' => {
        while self.chars.next_if(|c| *c == ' ' || *c == '\t').is_some() {}
        return self.next();
    }
    // parse potentially multi-character number tokens
    c if c.is_numeric() => {
        let mut str = c.to_string();
        while let Some(ch) = self.chars.next_if(|c| c.is_numeric() || *c == '.') {
            str.push(ch);
        }
        Token::Number(f64::from_str(&str).unwrap())
    }
    // other cases ...
}
```

</details>

As always, for a full solution check out the [`solutions` branch of the GitHub repository](https://github.com/mainmatter/containers-are-dead/tree/solutions).

[`.chars()`]: https://doc.rust-lang.org/std/primitive.str.html#method.chars
[`.peekable()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable
[`.next_if()`]: https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.next_if
[`f64::from_str`]: https://doc.rust-lang.org/std/primitive.f64.html#method.from_str

[^1]: Reference types also exist but are not supported across all WebAssembly runtimes just yet.
