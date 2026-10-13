use serde::{Deserialize, Serialize};
use spin_sdk::http::{Request, Response};
use spin_sdk::http_service;

// curl --request POST --data 'a = 10' http://localhost:3000/
// curl --request POST --data 'a * 2' http://localhost:3000/?session=<ID>

#[derive(Deserialize)]
struct QueryParams {
    session: Option<i64>,
}

#[derive(Serialize)]
struct EvalResponse {
    session: i64,
    result: Option<f64>,
}

// This component should:
// 1. parse and evaluate input expressions as before
// 2. it should NOT store assigned <ident> => value pairs in the KV store but instead in the provided SQLite database's variables table.
//    see the `migration.sql` file for the shape of the table. Reassigning a variable replaces its value.
// 3. It should support session IDs to handle multiple concurrent users. A session ID may be provided via the query parameters (see the struct above)
//    and IF PRESENT should be used to select the matching rows in the table. IF NOT PRESENT we should generate one (using the sessions table).
//    An unknown session ID is a client error (404).
// 4. Variables stored in one session MUST NOT be visible in another session
// 5. Respond with the `EvalResponse` struct above serialized as JSON
// If you get stuck, ask a neighbor or grab a trainer!
#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<Response<String>> {
    todo!()
}
