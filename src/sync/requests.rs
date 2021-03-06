//! Functionality for sending requests to Slack.

use std::{borrow::Borrow, error};

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// If you do not have a custom client to integrate with and just want to send requests, use
/// the [`default_client()`] function to get a simple request sender.
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    fn send<I, K, V, S>(&self, method: S, params: I) -> Result<String, Self::Error>
    where
        I: IntoIterator + Send,
        K: AsRef<str>,
        V: AsRef<str>,
        I::Item: Borrow<(K, V)>,
        S: AsRef<str> + Send;
}

#[cfg(feature = "reqwest_blocking")]
mod reqwest_support {

    pub use self::reqwest::Error;
    use reqwest_ as reqwest;
    use std::borrow::Borrow;

    use super::SlackWebRequestSender;

    type Client = reqwest::blocking::Client;

    impl SlackWebRequestSender for Client {
        type Error = reqwest::Error;

        fn send<I, K, V, S>(&self, method_url: S, params: I) -> Result<String, Self::Error>
        where
            I: IntoIterator + Send,
            K: AsRef<str>,
            V: AsRef<str>,
            I::Item: Borrow<(K, V)>,
            S: AsRef<str> + Send,
        {
            let mut url = reqwest::Url::parse(method_url.as_ref()).expect("Unable to parse url");

            let (mut token, not_token): (Vec<(String, String)>, Vec<(String, String)>) = 
                params
                    .into_iter()
                    .map(|kv| {
                        let (k, v) = kv.borrow();
                        (k.as_ref().to_string(), v.as_ref().to_string())
                    })
                    .partition(|(k, _)| k == "token");

            url.query_pairs_mut().extend_pairs(not_token);
            let mut req = self.get(url);

            if token.len() >= 1 {
                let token = token.pop().unwrap().1;
                req = req.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token));
            }

            Ok(req.send()?.text()?)
        }
    }

    /// Provides a default `reqwest` client to give to the API functions to send requests.
    ///
    /// # Examples
    ///
    /// ```
    /// # let token = "some_token";
    /// let client = slack_api::requests::default_client().unwrap();
    /// let params = Default::default();
    /// let response = slack_api::channels::list(&client, &token, &params);
    /// ```
    pub fn default_client() -> Result<Client, reqwest::Error> {
        Ok(Client::new())
    }
}

#[cfg(feature = "reqwest_blocking")]
pub use self::reqwest_support::*;
