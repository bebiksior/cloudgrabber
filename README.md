## CloudGrabber
CloudGrabber is a simple tool designed for Bug Bounty hunters. This tool is written in Rust and is used to gather subdomains or IP addresses associated with a company.

**[NOTE]** This has been written by ChatGPT in 90%, and I am still learning the Rust language. If you spot any mistakes in the code, feel free to contribute to the project by submitting pull requests.

## Features
- Grab IP addresses related a specific company
- Enumerate subdomains related to a specific company

## Usage
You can specify the company name and the type of output you want from the tool.

```bash
cloudgrabber --company <company-name> --output <output-type>
```
The `<output-type>` can be:
- ips
- dns-name
- common-name
- domains (dns-name AND common-name)

Example:
```bash
cloudgrabber --company="Snap Inc." --output="domains" 
```

## Build
To build this tool, you need to have Rust installed in your system. Once you have Rust, you can build the tool using the following command.

```bash
git clone https://github.com/bebiksior/cloudgrabber.git
cd cloudgrabber
cargo build --release
mv target/release/cloudgrabber /bin/cloudgrabber
```

## Contributions
Feel free to contribute to the project by creating issues or submitting pull requests.

## Disclaimer
This tool is meant to be used for legal purposes only. The creator of this tool takes no responsibility for any misuse of the tool.

## Credit
Thanks to Trickest for sharing up-to-date map of the cloud landscape for every major cloud service provider.
https://github.com/trickest/cloud
