# Databases

Key value stores are simple and easy to use, but they have two big disadvantages: no support for more complex data types and queries AND they are global. Meaning different users of your API could override each other's variables!

For "proper" persistent data storage `spin` also supports SQL databases like [`SQLite`]. To use SQL databases in Spin, you need to declare access in your `spin.toml` manifest and then use the bindings from the `spin_sdk` crate. In our case that is the [`spin_sdk::sqlite`] module.

## Database Configuration

First, add database access to your `spin.toml`:

```diff
[component.databases]
source = "../../../target/wasm32-wasip2/debug/databases.wasm"
+sqlite_databases = ["default"]
```

Spin creates the `default` database on first use but no tables. Pass the migration at startup (the tests do the same):

```bash
spin up --build --sqlite @migration.sql
```

Like the key value store this is a Spin interface, `spin:sqlite/sqlite@3.1.0` in the imports. Here not even a draft standard exists: [`wasi:sql`] is phase 1 and dormant since early 2024. The portable alternative is to compile SQLite _into_ the component (`rusqlite` with `bundled` builds for `wasm32-wasip2`), in memory or on top of `wasi:filesystem`.

## Connecting to the Database

Once you've configured database access in your manifest, you can establish a connection using the `Connection::open_default()` method. This opens the SQLite database that Spin provides to your component:

```rust
use spin_sdk::sqlite::{Connection, Value};

let connection = Connection::open_default().await?;

// do something with the connection...
```

## Inserting Data

You can insert data into your database using the [`execute`] method that takes a SQL query string and an iterator of parameters. Parameters use `?` placeholders in the query and are provided as `Value` enum variants to ensure type safety.

```rust
use spin_sdk::sqlite::{Connection, Value};

let connection = Connection::open_default().await?;

// Insert a new user passing the parameters we set up
connection
    .execute(
        "INSERT INTO users (name, age) VALUES (?, ?)",
        [Value::Text("john".to_string()), Value::Integer(20)],
    )
    .await?
    .result()
    .await?;
```

Careful: `execute` returns once the statement is _started_. The [`QueryResult`] is a stream of rows plus a future for the outcome, `result()` awaits the latter. If you forget, the `INSERT` call may still be running when your handler responds.

## Retrieving Data

The [`execute`] method returns a [`QueryResult`], a stream of rows. [`next()`] yields one [`RowResult`] at a time; [`get()`] converts a column (by index) from [`SQLite`]'s storage types to Rust types. Column names are available via [`columns()`].

```rust
use spin_sdk::sqlite::{Connection, Value};

let connection = Connection::open_default().await?;

let mut rows = connection
    .execute(
        "SELECT name, age FROM users WHERE age > ?",
        [Value::Integer(18)],
    )
    .await?;

while let Some(row) = rows.next().await {
    let name: &str = row.get(0).unwrap();
    let age: i64 = row.get(1).unwrap();
    println!("User: {}, Age: {}", name, age);
}
rows.result().await?;
```

`result()` also reports errors while reading rows. [`collect()`] does both in one go.

## Exercise

Move the variables into SQLite and add _sessions_ so users stop sharing them. See `src/lib.rs` and `migration.sql`.

[`SQLite`]: https://www.sqlite.org
[`wasi:sql`]: https://github.com/WebAssembly/wasi-sql
[`spin_sdk::sqlite`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/index.html
[`execute`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.Connection.html#method.execute
[`QueryResult`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.QueryResult.html
[`next()`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.QueryResult.html#method.next
[`columns()`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.QueryResult.html#method.columns
[`collect()`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.QueryResult.html#method.collect
[`RowResult`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.RowResult.html
[`get()`]: https://docs.rs/spin-sdk/latest/spin_sdk/sqlite/struct.RowResult.html#method.get
