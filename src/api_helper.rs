use std::collections::HashMap;

static PASTE_BIN_URL: &str = "https://pastebin.com/api/api_post.php";

fn get_post_parameters(api_key: String, file_data: String) -> HashMap<&'static str, String> {
    let params = HashMap::from([
        ("api_dev_key", api_key),
        ("api_paste_code", file_data),
        ("api_option", "paste".to_string()),
    ]);

    params
}

pub async fn post_pastebin(
    api_key: String,
    file_path: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = get_post_parameters(api_key, file_path);
    let res = client.post(PASTE_BIN_URL).form(&params).send().await;
    res
}
