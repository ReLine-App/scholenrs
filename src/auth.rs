use std::{self,collections::HashMap};

use crate::{client, error::{ScholenResult, ScholenError}};
use ureq;
use url::Url;

pub fn get_token(username: &str, password: &str) -> ScholenResult<String> {
    let agent: ureq::Agent = ureq::AgentBuilder::new()
      .redirects(0)
      .build();

    let resp = match agent.post(&(client::URL.to_owned() + "/portaal/login.phtml"))
        .send_form(&[
            ("pass_type", "plain"),
            ("username", username),
            ("password", password)
        ]) {
            Ok(resp) => resp,
            Err(_) => return Err(ScholenError::ApplicationUnreachable)
        };
    
    if let Some(location) = resp.header("Location") {
        let url = match Url::parse(location) {
            Ok(url) => url,
            Err(_) => return Err(ScholenError::InvalidCredentials)
        };

        let query: HashMap<String, String> =  url.query_pairs().into_owned().collect();
        
        if let Some(token) = query.get("password") {
            Ok(token.to_owned())
        } else {
            Err(ScholenError::InvalidCredentials)
        }

    } else {
        Err(ScholenError::InvalidCredentials)
    }
}