use std::collections::HashMap;

static PASTE_BIN_URL: &str = "https://pastebin.com/api/api_post.php";

fn get_post_parameters(api_key: &str) -> HashMap<&'static str, &'static str>{
    let params = HashMap::from([
    ("api_dev_key", api_key),
    ("api_paste_code", "my text"),
    ("api_option", "paste"),
]);
    params
}

pub async fn send_paste_request() -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = get_post_parameters();
    let res = client.post(PASTE_BIN_URL).form(&params).send().await;
    res
}
