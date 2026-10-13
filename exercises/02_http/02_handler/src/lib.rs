use spin_sdk::http::{Request, Response};
use spin_sdk::http_service;

// This component should:
// 1. accept HTTP POST requests only, other methods get a 405 response
// 2. read the input from the request body
// 3. pass that input into the calculator we implemented earlier, and return the evaluated result as text
// 4. respond with a 400 for invalid input instead of panicking
// you can check your work using curl: curl --request POST --data '10 / 2' http://localhost:3000/
// attention: you need to start the spin server with `spin up --build` for that, the tests do it themselves
#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<Response<String>> {
    todo!()
}
