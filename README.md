# PasteBinGen

A command-line interface for creating pastes on PasteBin easily. 

| Feature                 | Status |
|-------------------------|--------|
| Set syntax highlighting | Done   |
| Set file name           | Done   |

## Summary

1. [Requirements](#requirements)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Example](#example)
5. [Help](#help)
6. [Limitations](#limitations)
7. [License](#license)

### Requirements

   1. [Rust](https://www.rust-lang.org/tools/install)

### Installation

   ```
cargo install pastebin_gen
   ```

### Usage
   1. **API key of PasteBin.**

      If it's the first time you're using the program, you should configure the API key of PasteBin. In order to do that, you need to create an account on [PasteBin](https://pastebin.com/signup). After that, go to the [API's documentation](https://pastebin.com/doc_api) and find the section **"Your Unique Developer API Key"**. Copy the key above and set the api key with the flag `api-key` alongside with the path of the file that you want to upload to PasteBin.
      **If you run the program, but the API key is not avaiable, the program will crash.**

   2. **Paste format options**

      If you want to see the format options for enabling syntax highlighting, see that [list](https://pastebin.com/doc_api#5). If you don't set any formating option, it will have no syntax highlighting.

### Example

   ```
pastebin_gen --api-key YOUR_API_KEY --path path/to/file --paste-format python --paste-name my_file_name
   ```
   
   **WARNING**: `--api-key` is not necessary unless you're setting it for the first time or you want to replace the current one. Additionally, `--paste-name` and `paste-format` are optional flags. 

### Help

   The usage is pretty straightforward. However, if you need help, try to run `pastebin_gen --help`.
   
### Limitations
   1. **Maximum 10 pastes per 24h**
   
      The PasteBin's API does not allow a free user to create more than 10 pastes within the 24 hour limit.

### License
This project is licensed under the [MIT](LICENSE) license.
