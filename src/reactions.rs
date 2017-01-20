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

//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Adds a reaction to an item.
///
/// Wraps https://api.slack.com/methods/reactions.add
pub fn add<R: SlackWebRequestSender>(client: &R,
           token: &str,
           name: &str,
           file: Option<&str>,
           file_comment: Option<&str>,
           channel: Option<&str>,
           timestamp: Option<&str>)
           -> ApiResult<AddResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(channel) = channel {
        params.insert("channel", channel);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    let response = try!(client.send_authed("reactions.add", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AddResponse;

/// Gets reactions for an item.
///
/// Wraps https://api.slack.com/methods/reactions.get
pub fn get<R: SlackWebRequestSender>(client: &R,
           token: &str,
           file: Option<&str>,
           file_comment: Option<&str>,
           channel: Option<&str>,
           timestamp: Option<&str>,
           full: Option<&str>)
           -> ApiResult<GetResponse> {
    let mut params = HashMap::new();
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(channel) = channel {
        params.insert("channel", channel);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    if let Some(full) = full {
        params.insert("full", full);
    }
    let response = try!(client.send_authed("reactions.get", token, params));
    parse_slack_response(response, true)
}

// This is an Item as returned by `reactions.list`, but instead of being a
// nested object like all
// of the other endpoints is instead inlined at the top level.
pub type GetResponse = super::Item;

/// Lists reactions made by a user.
///
/// Wraps https://api.slack.com/methods/reactions.list
pub fn list<R: SlackWebRequestSender>(client: &R, token: &str, user: Option<&str>, full: Option<&str>, count: Option<u32>, page: Option<u32>) -> ApiResult<ListResponse> {
    let count = count.map(|c| c.to_string());
    let page = page.map(|p| p.to_string());
    let mut params = HashMap::new();
    if let Some(user) = user {
        params.insert("user", user);
    }
    if let Some(full) = full {
        params.insert("full", full);
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    if let Some(ref page) = page {
        params.insert("page", page);
    }
    let response = try!(client.send_authed("reactions.list", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub items: Vec<super::Item>,
    pub paging: super::Pagination,
}

/// Removes a reaction from an item.
///
/// Wraps https://api.slack.com/methods/reactions.remove
pub fn remove<R: SlackWebRequestSender>(client: &R,
              token: &str,
              name: &str,
              file: Option<&str>,
              file_comment: Option<&str>,
              channel: Option<&str>,
              timestamp: Option<&str>)
              -> ApiResult<RemoveResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    if let Some(file) = file {
        params.insert("file", file);
    }
    if let Some(file_comment) = file_comment {
        params.insert("file_comment", file_comment);
    }
    if let Some(channel) = channel {
        params.insert("channel", channel);
    }
    if let Some(timestamp) = timestamp {
        params.insert("timestamp", timestamp);
    }
    let response = try!(client.send_authed("reactions.remove", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RemoveResponse;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Item;
    use super::super::Message;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = add(&client,
                         "TEST_TOKEN",
                         "thumbsup",
                         None,
                         None,
                         Some("C1234567890"),
                         Some("1234567890.123456"));
        assert!(result.is_err());
    }

    #[test]
    fn add_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = add(&client,
                         "TEST_TOKEN",
                         "thumbsup",
                         None,
                         None,
                         Some("C1234567890"),
                         Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn get_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "type": "message",
            "channel": "C1234567890",
            "message": {
                "type": "message",
                "channel": "C1234567890",
                "user": "U2147483697",
                "text": "Hello world",
                "ts": "1234567890.123456",
                "reactions": [
                    {
                        "name": "astonished",
                        "count": 3,
                        "users": [ "U1", "U2", "U3" ]
                    },
                    {
                        "name": "clock1",
                        "count": 2,
                        "users": [ "U1", "U2", "U3" ]
                    }
                ]
            }
        }"#);
        let result = get(&client,
                         "TEST_TOKEN",
                         None,
                         None,
                         Some("C1234567890"),
                         Some("1234567890.123456"),
                         None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().clone() {
            Item::Message { channel: c, message: m } => {
                assert_eq!(c, "C1234567890");
                match *m.clone() {
                    Message::Standard { ts: _, channel: _, user: _, text, is_starred: _,
                         pinned_to: _, reactions, edited: _, attachments: _, .. } => {
                        assert_eq!(text.unwrap(), "Hello world");
                        assert_eq!(reactions.unwrap()[0].name, "astonished");
                    }
                    _ => panic!("Message decoded into incorrect variant."),
                }
            }
            _ => panic!("Item decoded into incorrect variant."),
        }
    }

    #[test]
    fn list_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "items": [
                {
                    "type": "message",
                    "channel": "C1234567890",
                    "message": {
                        "type": "message",
                        "channel": "C1234567890",
                        "user": "U2147483697",
                        "text": "Hello world",
                        "ts": "1234567890.123456",
                        "reactions": [
                            {
                                "name": "astonished",
                                "count": 3,
                                "users": [ "U1", "U2", "U3" ]
                            },
                            {
                                "name": "clock1",
                                "count": 2,
                                "users": [ "U1", "U2", "U3" ]
                            }
                        ]
                    }
                },
                {
                    "type": "file",
                    "file": {
                        "id": "F12345678",
                        "created": 1444929467,
                        "timestamp": 1444929467,
                        "name": "test_img.png",
                        "title": "test_img",
                        "mimetype": "image\/png",
                        "filetype": "png",
                        "pretty_type": "PNG",
                        "user": "U12345678",
                        "editable": false,
                        "size": 16153,
                        "mode": "hosted",
                        "is_external": false,
                        "external_type": "",
                        "is_public": true,
                        "public_url_shared": false,
                        "display_as_bot": false,
                        "username": "",
                        "url": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/test_img.png",
                        "url_download": "https:\/\/slack-files.com\/files-pub\/PUBLIC-TEST-GUID\/download\/test_img.png",
                        "url_private": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/test_img.png",
                        "url_private_download": "https:\/\/files.slack.com\/files-pri\/PRIVATE-ID\/download\/test_img.png",
                        "thumb_64": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_64.png",
                        "thumb_80": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_80.png",
                        "thumb_360": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_360.png",
                        "thumb_360_w": 360,
                        "thumb_360_h": 28,
                        "thumb_480": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_480.png",
                        "thumb_480_w": 480,
                        "thumb_480_h": 37,
                        "thumb_160": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_160.png",
                        "thumb_720": "https:\/\/slack-files.com\/files-tmb\/PRIVATE-TEST-GUID\/test_img_720.png",
                        "thumb_720_w": 720,
                        "thumb_720_h": 56,
                        "image_exif_rotation": 1,
                        "original_w": 895,
                        "original_h": 69,
                        "permalink": "https:\/\/test-team.slack.com\/files\/testuser\/F12345678\/test_img.png",
                        "permalink_public": "https:\/\/slack-files.com\/PUBLIC-TEST-GUID",
                        "channels": [
                            "C12345678"
                        ],
                        "groups": [

                        ],
                        "ims": [

                        ],
                        "comments_count": 0,
                        "reactions": [
                            {
                                "name": "thumbsup",
                                "count": 1,
                                "users": [ "U1" ]
                            }
                        ]
                    }
                }
            ],
            "paging": {
                "count": 100,
                "total": 5,
                "page": 1,
                "pages": 1
            }
        }"#);
        let result = list(&client, "TEST_TOKEN", None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().items[1] {
            Item::File { file: ref f } => {
                assert_eq!(f.id, "F12345678");
                assert_eq!(f.reactions.as_ref().unwrap()[0].name, "thumbsup");
            }
            _ => panic!("Item decoded into incorrect variant."),
        }
    }

    #[test]
    fn remove_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = remove(&client,
                            "TEST_TOKEN",
                            "thumbsup",
                            None,
                            None,
                            Some("C1234567890"),
                            Some("1234567890.123456"));
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
