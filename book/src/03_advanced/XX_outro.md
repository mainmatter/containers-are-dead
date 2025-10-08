# Outro

In this workshop we built a calculator API that parses and evaluates arithmetic expressions and persists session state across requests using SQLite. In the process we have seen how WebAssembly can be used to built server side applications today, seen the benefits and experienced the rough edges.

If you want to continue playing with our "calculator-as-a-service" here are a few ideas:

- Add a queryable history feature that stores each calculation with timestamps, allowing users to retrieve their calculation history.
- Extend this history feature to allow users to share expressions.
- Add a simple frontend using HTML, JS, and CSS.
- Implement that frontend using a Rust+WebAssembly frontend framework!

## Further Reading

- `JCO` for running components on the web: https://github.com/bytecodealliance/jco
- [Component Model Specification](https://github.com/WebAssembly/component-model)
- [WebAssembly Blast Zones](https://github.com/WebAssembly/component-model/blob/main/design/mvp/FutureFeatures.md#blast-zones)
