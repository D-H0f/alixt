use crate::models::{
    config::{Config, Method, Request, Run, Scheme},
    error::AlixtError,
    plan::{ExecuteRequest, RunPlan, TestPlan},
};

pub fn parse(content: String) -> Result<TestPlan, AlixtError> {
    let config: Config = toml::from_str(&content)?;

    let mut plan = TestPlan::new();

    for run in config.run {
        plan.runs.push(build_run(run)?);
    }
    Ok(plan)
}

fn build_run(run: Run) -> Result<RunPlan, AlixtError> {
    let mut run_plan = RunPlan::new(run.name);

    for mut request in run.request {
        // inheritance checks
        if request.headers.is_none() && run.headers.is_some() {
            request.headers = run.headers.clone();
        }
        if request.method.is_none() {
            if run.method.is_none() {
                return Err(AlixtError::Config(format!(
                    "No default or explicit method present for {}",
                    request.name
                )));
            }
            request.method = run.method.clone();
        }
        if request.scheme.is_none() {
            if run.scheme.is_none() {
                return Err(AlixtError::Config(format!(
                    "No default or explicit scheme present for {}",
                    request.name
                )));
            }
            request.scheme = run.scheme.clone();
        }
        if request.host.is_none() {
            if run.host.is_none() {
                return Err(AlixtError::Config(format!(
                    "No default or explicit host present for {}",
                    request.name
                )));
            }
            request.host = run.host.clone();
        }
        if request.port.is_none() {
            request.port = run.port.clone();
        }
        if request.path.is_none() {
            // if neither the request nor run specify a path, it defaults to "/"
            request.path = run.path.clone();
        }
        if request.body.is_none() {
            request.body = run.body.clone();
        }

        run_plan.requests.push(build_request(request)?);
    }

    Ok(run_plan)
}

fn build_request(request: Request) -> Result<ExecuteRequest, AlixtError> {
    let mut url = format!(
        "{}://{}",
        request.scheme.unwrap_or_else(|| Scheme::Http),
        request.host.ok_or_else(|| AlixtError::Config(format!(
            "Internal Error: Request {} missing host, got past checks",
            request.name
        )))?
    );
    if let Some(port) = request.port {
        url = format!("{}:{}", url, port);
    }
    if let Some(path) = request.path {
        url = format!("{}{}", url, path);
    } else {
        url = format!("{}/", url)
    }

    let method = match request.method.ok_or_else(|| {
        AlixtError::Config(format!(
            "Internal Error: Request {} missing method, got past checks",
            request.name
        ))
    })? {
        Method::Get => reqwest::Method::GET,
        Method::Put => reqwest::Method::PUT,
        Method::Post => reqwest::Method::POST,
        Method::Patch => reqwest::Method::PATCH,
        Method::Delete => reqwest::Method::DELETE,
    };
    let request_plan = ExecuteRequest {
        name: request.name,
        url,
        method,
        body: request.body,
        headers: request.headers,
        capture: request.capture,
        assert: request.assert,
    };
    Ok(request_plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_inheritance() {
        let toml_input = r#"
        [[run]]
        name = "Base Run"
        method = "Get"
        scheme = "Http"
        host = "0.0.0.0"
        port = 4848
        path = "/hello/world"

        [[run.request]]
        name = "Inherit from run"

        [[run.request]]
        name = "Overrides everything"
        method = "Post"
        scheme = "Https"
        host = "api.example.com"
        port = 80
        path = "/a/new/path"
        "#;

        let plan = match parse(toml_input.to_string()) {
            Ok(plan) => plan,
            Err(e) => panic!("ERROR: {e:#?}"),
        };

        let req_inherit = &plan.runs[0].requests[0];
        assert_eq!(req_inherit.url, "http://0.0.0.0:4848/hello/world");
        assert_eq!(req_inherit.method, reqwest::Method::GET);

        let req_override = &plan.runs[0].requests[1];
        assert_eq!(req_override.url, "https://api.example.com:80/a/new/path");
        assert_eq!(req_override.method, reqwest::Method::POST);
    }
}
