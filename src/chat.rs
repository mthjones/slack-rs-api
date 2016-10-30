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

//! Post chat messages to Slack.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Deletes a message.
///
/// Wraps https://api.slack.com/methods/chat.delete
pub fn delete<R: SlackWebRequestSender>(client: &R, token: &str, ts: &str, channel: &str) -> ApiResult<DeleteResponse> {
    let mut params = HashMap::new();
    params.insert("ts", ts);
    params.insert("channel", channel);
    let response = try!(client.send_authed("chat.delete", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct DeleteResponse {
    pub channel: String,
    pub ts: String,
}

/// Sends a message to a channel.
///
/// Wraps https://api.slack.com/methods/chat.postMessage
pub fn post_message<R: SlackWebRequestSender>(client: &R,
                    token: &str,
                    channel: &str,
                    text: &str,
                    username: Option<&str>,
                    as_user: Option<bool>,
                    parse: Option<&str>,
                    link_names: Option<bool>,
                    attachments: Option<&str>,
                    unfurl_links: Option<bool>,
                    unfurl_media: Option<bool>,
                    icon_url: Option<&str>,
                    icon_emoji: Option<&str>)
                    -> ApiResult<PostMessageResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("text", text);
    if let Some(username) = username {
        params.insert("username", username);
    }
    if let Some(as_user) = as_user {
        params.insert("as_user",
                      if as_user {
                          "true"
                      } else {
                          "false"
                      });
    }
    if let Some(parse) = parse {
        params.insert("parse", parse);
    }
    if let Some(link_names) = link_names {
        params.insert("link_names",
                      if link_names {
                          "1"
                      } else {
                          "0"
                      });
    }
    if let Some(attachments) = attachments {
        params.insert("attachments", attachments);
    }
    if let Some(unfurl_links) = unfurl_links {
        params.insert("unfurl_links",
                      if unfurl_links {
                          "true"
                      } else {
                          "false"
                      });
    }
    if let Some(unfurl_media) = unfurl_media {
        params.insert("unfurl_media",
                      if unfurl_media {
                          "true"
                      } else {
                          "false"
                      });
    }
    if let Some(icon_url) = icon_url {
        params.insert("icon_url", icon_url);
    }
    if let Some(icon_emoji) = icon_emoji {
        params.insert("icon_emoji", icon_emoji);
    }
    let response = try!(client.send_authed("chat.postMessage", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct PostMessageResponse {
    pub ts: String,
    pub channel: String,
    pub message: super::Message,
}

/// Updates a message.
///
/// Wraps https://api.slack.com/methods/chat.update
pub fn update<R: SlackWebRequestSender>(client: &R,
              token: &str,
              ts: &str,
              channel: &str,
              text: &str,
              attachments: Option<&str>,
              parse: Option<&str>,
              link_names: Option<bool>)
              -> ApiResult<UpdateResponse> {
    let mut params = HashMap::new();
    params.insert("ts", ts);
    params.insert("channel", channel);
    params.insert("text", text);
    if let Some(attachments) = attachments {
        params.insert("attachments", attachments);
    }
    if let Some(parse) = parse {
        params.insert("parse", parse);
    }
    if let Some(link_names) = link_names {
        params.insert("link_names",
                      if link_names {
                          "1"
                      } else {
                          "0"
                      });
    }
    let response = try!(client.send_authed("chat.update", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct UpdateResponse {
    pub channel: String,
    pub ts: String,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Message;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = post_message(&client,
                                  "TEST_TOKEN",
                                  "TEST_CHANNEL",
                                  "Test message",
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None);
        assert!(result.is_err());
    }

    #[test]
    fn delete_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": "C024BE91L",
            "ts": "1401383885.000061"
        }"#);
        let result = delete(&client, "TEST_TOKEN", "1401383885.000061", "C024BE91L");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().ts, "1401383885.000061");
    }

    #[test]
    fn post_message_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "ts": "1405895017.000506",
            "channel": "C024BE91L",
            "message": {
                "type": "message",
                "user": "U024BE7LH",
                "text": "Test message",
                "ts": "1444078138.000084"
            }
        }"#);
        let result = post_message(&client,
                                  "TEST_TOKEN",
                                  "TEST_CHANNEL",
                                  "Test message",
                                  None,
                                  Some(true),
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().message {
            Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(text.unwrap(), "Test message");
            }
            _ => panic!("Message decoded into incorrect variant."),
        }
    }

    #[test]
    fn bot_post_message_ok_response() {
        let sender = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "ts": "1405895017.000506",
            "channel": "C024BE91L",
            "message": {
                "type": "message",
                "text": "Test message",
                "ts": "1444078138.000084"
            }
        }"#);
        let result = post_message(&sender,
                                  "TEST_TOKEN",
                                  "TEST_CHANNEL",
                                  "Test message",
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None,
                                  None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().message.clone() {
            Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                assert_eq!(text.unwrap(), "Test message")
            }
            _ => panic!("Message decoded into incorrect variant."),
        }
    }

    #[test]
    fn update_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": "C024BE91L",
            "ts": "1401383885.000061",
            "text": "Test message"
        }"#);
        let result = update(&client,
                            "TEST_TOKEN",
                            "TEST_CHANNEL",
                            "1401383885.000061",
                            "Test message",
                            None,
                            None,
                            None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().text, "Test message");
    }
}
