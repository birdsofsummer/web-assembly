// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct TopLevel {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "commit")]
    commit: TopLevelCommit,

    #[serde(rename = "_links")]
    links: Links,

    #[serde(rename = "protected")]
    protected: bool,

    #[serde(rename = "protection")]
    protection: Protection,

    #[serde(rename = "protection_url")]
    protection_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopLevelCommit {
    #[serde(rename = "sha")]
    sha: String,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "commit")]
    commit: CommitCommit,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "comments_url")]
    comments_url: String,

    #[serde(rename = "author")]
    author: PurpleAuthor,

    #[serde(rename = "committer")]
    committer: PurpleAuthor,

    #[serde(rename = "parents")]
    parents: Vec<Parent>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleAuthor {
    #[serde(rename = "login")]
    login: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "avatar_url")]
    avatar_url: String,

    #[serde(rename = "gravatar_id")]
    gravatar_id: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "followers_url")]
    followers_url: String,

    #[serde(rename = "following_url")]
    following_url: String,

    #[serde(rename = "gists_url")]
    gists_url: String,

    #[serde(rename = "starred_url")]
    starred_url: String,

    #[serde(rename = "subscriptions_url")]
    subscriptions_url: String,

    #[serde(rename = "organizations_url")]
    organizations_url: String,

    #[serde(rename = "repos_url")]
    repos_url: String,

    #[serde(rename = "events_url")]
    events_url: String,

    #[serde(rename = "received_events_url")]
    received_events_url: String,

    #[serde(rename = "type")]
    author_type: String,

    #[serde(rename = "site_admin")]
    site_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CommitCommit {
    #[serde(rename = "author")]
    author: FluffyAuthor,

    #[serde(rename = "committer")]
    committer: FluffyAuthor,

    #[serde(rename = "message")]
    message: String,

    #[serde(rename = "tree")]
    tree: Tree,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "comment_count")]
    comment_count: i64,

    #[serde(rename = "verification")]
    verification: Verification,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyAuthor {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "email")]
    email: String,

    #[serde(rename = "date")]
    date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tree {
    #[serde(rename = "sha")]
    sha: String,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Verification {
    #[serde(rename = "verified")]
    verified: bool,

    #[serde(rename = "reason")]
    reason: String,

    #[serde(rename = "signature")]
    signature: Option<serde_json::Value>,

    #[serde(rename = "payload")]
    payload: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "sha")]
    sha: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: String,

    #[serde(rename = "html")]
    html: String,
}

#[derive(Serialize, Deserialize)]
pub struct Protection {
    #[serde(rename = "enabled")]
    enabled: bool,

    #[serde(rename = "required_status_checks")]
    required_status_checks: RequiredStatusChecks,
}

#[derive(Serialize, Deserialize)]
pub struct RequiredStatusChecks {
    #[serde(rename = "enforcement_level")]
    enforcement_level: String,

    #[serde(rename = "contexts")]
    contexts: Vec<Option<serde_json::Value>>,
}
