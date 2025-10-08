const TEMPLATE_TOML: &str = r#"
# This is an example for the API tester.
# The file can contain multiple [[runs]], each with its own set of requests.

# A 'run' is a sequence of blocking tests. A failure in one run will not
# prevent the next run from starting. This is useful for grouping tests
# by category (e.g., "Account Tests", "Post Tests").
[[runs]]
# A name for the test run or suite.
name = "Account Management"

# You can define default values for all requests within this run.
# These can be overridden by individual requests.
url = "0.0.0.0"
port = 7878

  # Each request is defined by a [[runs.requests]] table
  [[runs.requests]]
  # A descriptive name for the test case
  name = "Get All Accounts"

  # The HTTP method to use. Options are: Get, Post, Put, Delete
  # This request uses the default url and port from the run.
  method = "Get"

  # The path on the server to target
  target = "/accounts"

  # The body of the request (for Post and Put)
  # Use TOML's multi-line strings for JSON
  # body = '''
  # { "username": "Jeff", "role": "User" }
  # '''

  # Assertions to check against the response
  [runs.requests.assert]
  # The expected HTTP status code
  status = 200
  # If true, the test run will stop if the assertion fails
  # If false, it will report the failure and continue
  breaking = true
  # Optional: The expected response body.
  # Both the expected body, and the received body, will be parsed
  # as JSON for comparison, ignoring formatting and key order.
  body = '''
[
    {
        "id": 1,
        "username": "testuser",
        "role": "User"
    }
]
  '''

  [[runs.requests]]
  name = "Get a non-existent account"
  method = "Get"
  # This request also uses the default url and port.
  target = "/accounts/999"

  [runs.requests.assert]
  status = 404
  breaking = false

  [[runs.requests]]
  name = "Test an override"
  method = "Get"
  port = 8080 # This OVERRIDES the run default (7878) for this request only
  target = "/health"

  [runs.requests.assert]
  status = 200
  breaking = false

# You can define another run here.
# [[runs]]
# name = "Another Test Suite"
# ...
"#;

pub fn generate() -> Result<(), super::Error> {
    std::fs::write("test_requests.toml", TEMPLATE_TOML)?;
    println!("Template file created: 'test_requests.toml'");
    Ok(())
}
