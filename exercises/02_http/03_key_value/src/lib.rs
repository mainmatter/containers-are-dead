use spin_sdk::http::{Request, Response};
use spin_sdk::http_service;

// curl --request POST --data 'a = 10' http://localhost:3000/
// curl --request POST --data 'a * 2' http://localhost:3000/

// This component should do a number of things:
// 1. parse and evaluate request bodies as before
// 2. extend the parser to support alphabetical *identifiers* such as `a` or `foo`
// 3. if the request body starts with `<ident> = <right hand side>` that should NOT evaluate to a result,
//    but instead assign the value of the *right hand side* expression to the identifier and respond with an empty body.
// 4. Store these <ident> => value mappings in the spin key value store
// 5. If an expression contains an identifier in any other valid place (instead of a number) attempt to look
//    up the value from the key value store. Unknown identifiers are a client error (400).
// If you get stuck, ask a neighbor or grab a trainer!
#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<Response<String>> {
    todo!()
}
