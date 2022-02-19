use std::collections::HashMap;

static PASTE_BIN_URL: &str = "https://pastebin.com/api/api_post.php";

fn get_post_parameters<'a>(
    api_key: &'a str,
    file_data: &'a str,
    paste_format: &'a str,
    paste_name: &'a str,
) -> HashMap<&'static str, &'a str> {
    HashMap::from([
        ("api_dev_key", api_key),
        ("api_paste_code", file_data),
        ("api_paste_format", paste_format),
        ("api_paste_name", paste_name),
        ("api_option", "paste"),
    ])
}

pub async fn post(
    api_key: &str,
    file_path: &str,
    paste_format: &str,
    paste_name: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = get_post_parameters(api_key, file_path, paste_format, paste_name);
    let res = client.post(PASTE_BIN_URL).form(&params).send().await;
    res
}
