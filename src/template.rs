const TEMPLATE_TOML: &str = r#"
# This is an example for the API tester.
# The file can contain multiple [[run]], each with its own set of request.

# A 'run' is a sequence of blocking tests. A failure in one run will not
# prevent the next run from starting. This is useful for grouping tests
# by category (e.g., "Account Tests", "Post Tests").
[[run]]
# A name for the test run or suite.
name = "Account Management"

# You can define default values for all request within this run.
# These can be overridden by individual request.
url = "0.0.0.0"
port = 7878

  # Each request is defined by a [[run.request]] table
  [[run.request]]
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
  [run.request.assert]
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

  [[run.request]]
  name = "Get a non-existent account"
  method = "Get"
  # This request also uses the default url and port.
  target = "/accounts/999"

  [run.request.assert]
  status = 404
  breaking = false

  [[run.request]]
  name = "Test an override"
  method = "Get"
  port = 8080 # This OVERRIDES the run default (7878) for this request only
  target = "/health"

  [run.request.assert]
  status = 200
  breaking = false

# You can define another run here.
# [[run]]
# name = "Another Test Suite"
# ...
"#;

pub fn generate() -> Result<(), super::Error> {
    std::fs::write("test_request.toml", TEMPLATE_TOML)?;
    println!("Template file created: 'test_request.toml'");
    Ok(())
}
