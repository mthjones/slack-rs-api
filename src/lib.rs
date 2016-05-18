// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Low-level, direct interface for the [Slack Web
//! API](https://api.slack.com/methods).

pub extern crate hyper;
extern crate rustc_serialize;

#[cfg(test)] #[macro_use]
extern crate yup_hyper_mock;

use std::collections::HashMap;
use std::io::Read;

use rustc_serialize::{json, Decodable};

#[cfg(test)]
#[macro_use]
pub mod test_helpers {
    macro_rules! mock_slack_responder {
        ($name:ident, $json:expr) => {
            mock_connector!($name {
                "https://slack.com" => "HTTP/1.1 200 OK\r\n\r\n".to_owned() + $json
            });
        }
    }
}

mod types;
pub use self::types::*;

mod error;
pub use error::Error;

mod message_events;
pub use self::message_events::Message;

pub mod api;
pub mod auth;
pub mod channels;
pub mod chat;
pub mod emoji;
pub mod files;
pub mod groups;
pub mod im;
pub mod oauth;
pub mod pins;
pub mod reactions;
pub mod rtm;
pub mod search;
pub mod stars;
pub mod team;
pub mod users;

pub type ApiResult<T> = Result<T, Error>;

/// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
/// params. Returns the response body string after checking it has "ok": true, or an Error
fn make_api_call<'a, T: Decodable>(client: &hyper::Client, method: &str, custom_params: HashMap<&str, &'a str>) -> ApiResult<T> {
    let url_string = format!("https://slack.com/api/{}", method);
    let mut url = hyper::Url::parse(&url_string).expect("Unable to parse url");

    url.query_pairs_mut().extend_pairs(custom_params.into_iter());

    let response = try!(client.get(url).send());
    transform_api_result(response)
}

/// Make an API call to Slack that includes the configured token. Takes a map of parameters that
/// get appended to the request as query params. Returns the response body string after checking it
/// has `"ok": true`, or an Error
fn make_authed_api_call<'a, T: Decodable>(client: &hyper::Client, method: &str, token: &'a str, mut custom_params: HashMap<&str, &'a str>) -> ApiResult<T> {
    custom_params.insert("token", token);
    make_api_call(client, method, custom_params)
}

fn transform_api_result<T: Decodable>(mut res: hyper::client::response::Response) -> ApiResult<T> {
    let mut res_str = String::new();
    try!(res.read_to_string(&mut res_str));

    let raw_json = try!(json::Json::from_str(&res_str));
    let jobj = try!(raw_json.as_object()
                            .ok_or(Error::Api(format!("bad slack json response (not an object) {:?}", raw_json))));
    let ok = try!(jobj.get("ok")
                      .ok_or(Error::Api(format!("slack json reponse does not contain \"ok\" field {:?}",
                                                raw_json))));
    let is_ok = try!(ok.as_boolean()
                       .ok_or(Error::Api(format!("slack json reponse \"ok\" is not a boolean: {:?}", raw_json))));
    if !is_ok {
        return Err(Error::Api(format!("slack json reponse \"ok\" is not true: {:?}", raw_json)));
    }

    Ok(try!(json::decode(&res_str)))
}
