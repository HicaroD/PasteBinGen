# PasteBinGen
**WORK IN PROGRESS**

:memo: A command-line interface for creating pastes on PasteBin easily. 

## Summary

1. [Installation](#installation)
2. [Usage](#usage)
3. [License](#license)

### Installation

### Usage
   1. If it's the first time you're using the program, you should configure the API key of PasteBin.

   In order to do that, you need to create an account in [PasteBin website](https://pastebin.com/signup). After that, go to [here](https://pastebin.com/doc_api) and find the section **"Your Unique Developer API Key"**. Copy the key above and run the program `pastebin_gen --config YOUR_API_KEY_HERE`.
   Now you can use the program.

### Help

   The usage is pretty straightforward. However, if you need help, try to run `pastebin_gen --help`.
   
   ```
pastebin_gen 0.1.0
PasteBinGen
A simple CLI for writing PasteBin texts.

USAGE:
    pastebin_gen [OPTIONS]

OPTIONS:
    -a, --api-key <API_KEY>    PasteBin API key [default: default]
    -h, --help                 Print help information
    -V, --version              Print version information
   ```
   
### License
This project is licensed under the [MIT](LICENSE) license.
