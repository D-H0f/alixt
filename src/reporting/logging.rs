use serde_json::Value;

use crate::models::test_data::TestData;

pub fn standard_out(test_outcome: TestData) {
    println!("[TEST RESULTS]");
    for run in test_outcome.run_data {
        println!("\n[RUN]: '{}'", run.name);
        for req in run.outcomes {
            println!(
                "\n[REQUEST]: '{}',\nTarget: '{}',\nPassed: {},\nBreaking: {},\nDuration: {} seconds,",
                req.name,
                req.url,
                req.passing,
                req.breaking,
                req.duration.as_secs_f64(),
            );
            if let Some(body) = req.response_body {
                let body = if let Some(json) = serde_json::from_str::<Value>(&body).ok() {
                    format!("{}", serde_json::to_string_pretty(&json).unwrap_or(body.clone()))
                } else {
                    body
                };
                println!("Body = ```\n{}```", body);
            } else {
                println!("");
            }
        }
    }
}
