## rDNS

**rDNS** is a reverse DNS lookup utility, using which you can figure out the hostname associated with an IP address.


***

## Installation

### Method 1: Using Crates.io
**rDNS** is available on **[Crates.io](https://crates.io/crates/rDNS)**. So, if you have Rust installed on your system, you can simply install **rDNS** with the following command:

```bash
cargo install rDNS
```

### Method 2: Manual Build
You will need Cargo to perform the manual build for rDNS.
If you have Cargo installed, you can simply follow the steps below:
1. Clone this repository, `git clone https://github.com/TheBinitGhimire/rDNS`;
2. Go inside the folder, `cd rDNS`;
3. Use the `cargo build` command,
4. Go inside the newly-created **target** folder, and open the **debug** folder inside it, `cd target/debug`;
5. You will find **rDNS.exe** (on Microsoft Windows) or **rDNS** binary (on Linux).


***

## Usage

| Flag  | Description                        | Example             |
| ----- | ---------------------------------- | ------------------- |
|  -h   | Display help related to usage!     | rDNS -h             |
|  -t   | Scan a single IP!                  | rDNS -t 1.1.1.1     |
|  -f   | Scan a list of IPs from a file!    | rDNS -f list.txt    |
|  -V   | Display the version information!   | rDNS -V             |

***

### Use Case I: Defining an IP Address

```
rDNS -t 1.1.1.1
```

### Use Case II: Defining a File containing list of IP Addresses

```
rDNS -f list.txt
```


***

## Contributions and Feature Requests
<p align="center">
    <a href="https://github.com/TheBinitGhimire/rDNS/pulls"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge" /></a>
</p>


If you are interested in contributing in the development of <strong>rDNS</strong>, you can feel free to create a <strong>Pull Request</strong> with modifications in the original code, or you shall open up a new <strong>issue</strong>, and I will try to include the feature as requested.

There is no restriction on anyone for contributing to the development of <strong>rDNS</strong>. If you would like to contribute, you can feel free to do so.

