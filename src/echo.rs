//  {
//    "args": {}, 
//    "data": "", 
//    "files": {}, 
//    "form": {
//      "x": "1", 
//      "y": "2"
//    }, 
//    "headers": {
//      "Accept": "*/*", 
//      "Content-Length": "7", 
//      "Content-Type": "application/x-www-form-urlencoded", 
//      "Host": "httpbin.org", 
//      "User-Agent": "curl/7.61.1", 
//      "X-Amzn-Trace-Id": "Root=1-5e883829-74d9dae0d0aa1038090f50d2"
//    }, 
//    "json": null, 
//    "origin": "112.97.38.109", 
//    "url": "https://httpbin.org/post"
//  }







#[derive(Serialize, Deserialize)]
pub struct PostEcho {
    #[serde(rename = "args")]
    args: Args,

    #[serde(rename = "data")]
    data: String,

    #[serde(rename = "files")]
    files: Args,

    #[serde(rename = "form")]
    form: Form,

    #[serde(rename = "headers")]
    headers: Headers,

    #[serde(rename = "json")]
    json: Option<serde_json::Value>,

    #[serde(rename = "origin")]
    origin: String,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Args {
}

#[derive(Serialize, Deserialize)]
pub struct Form {
    #[serde(rename = "x")]
    x: String,

    #[serde(rename = "y")]
    y: String,
}

#[derive(Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "Accept")]
    accept: String,

    #[serde(rename = "Content-Length")]
    content_length: String,

    #[serde(rename = "Content-Type")]
    content_type: String,

    #[serde(rename = "Host")]
    host: String,

    #[serde(rename = "User-Agent")]
    user_agent: String,

    #[serde(rename = "X-Amzn-Trace-Id")]
    x_amzn_trace_id: String,
}




