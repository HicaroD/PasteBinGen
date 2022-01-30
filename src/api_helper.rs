use std::collections::HashMap;

static PASTE_BIN_URL: &str = "https://pastebin.com/api/api_post.php";

fn get_post_parameters(
    api_key: String,
    file_data: String,
    paste_format: String,
) -> HashMap<&'static str, String> {
    HashMap::from([
        ("api_dev_key", api_key),
        ("api_paste_code", file_data),
        ("api_paste_format", paste_format),
        ("api_option", "paste".to_string()),
    ])
}

pub async fn post_pastebin(
    api_key: String,
    file_path: String,
    paste_format: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = get_post_parameters(api_key, file_path, paste_format);
    let res = client.post(PASTE_BIN_URL).form(&params).send().await;
    res
}
