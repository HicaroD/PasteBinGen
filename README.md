# PasteBinGen

A command-line interface for creating pastes on PasteBin easily. 

| Feature                 | Status |
|-------------------------|--------|
| Set syntax highlighting | Done   |
| Set file name           | Done   |

## Summary

1. [Installation](#installation)
2. [Usage](#usage)
3. [Help](#help)
4. [Limitations](#limitations)
5. [License](#license)

### Installation

### Usage
   1. **API key of PasteBin.**

      If it's the first time you're using the program, you should configure the API key of PasteBin. In order to do that, you need to create an account on [PasteBin](https://pastebin.com/signup). After that, go to the [API's documentation](https://pastebin.com/doc_api) and find the section **"Your Unique Developer API Key"**. Copy the key above and configure the program `pastebin_gen --config YOUR_API_KEY_HERE`.
      **If you run the program, but the API key is not avaiable, the program will crash.**

   2. **Paste format options**

      If you want to see the format options for enabling syntax highlighting, see that [list](https://pastebin.com/doc_api#5). If you don't set any formating option, it will have no syntax highlighting.

### Help

   The usage is pretty straightforward. However, if you need help, try to run `pastebin_gen --help`.
   
   ```
pastebin_gen 0.1.0
PasteBinGen
A simple CLI for writing PasteBin texts.

USAGE:
    pastebin_gen [OPTIONS] --path <PATH>

OPTIONS:
    -a, --api-key <API_KEY>              PasteBin API key [default: default]
    -f, --paste-format <PASTE_FORMAT>    Syntax highlighting options [default: text]
    -h, --help                           Print help information
    -n, --paste-name <PASTE_NAME>        Paste name [default: untitled]
    -p, --path <PATH>                    Path to file that you want to upload
    -V, --version                        Print version information
   ```
   
### Limitations
   1. **Maximum 10 pastes per 24h**
   
      The PasteBin's API does not allow a free user to create more than 10 folders within the 24 hour limit.

### License
This project is licensed under the [MIT](LICENSE) license.
