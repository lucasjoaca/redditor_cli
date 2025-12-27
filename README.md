# Reddit CLI Monitor

A command-line interface (CLI) tool written in Rust that monitors specific subreddits, fetches post data at set intervals, and archives them locally in JSON format.

> **Academic Project** > This project was developed for the **Rust Programming** course at the **Faculty of Computer Science, Alexandru Ioan Cuza University of Iași (UAIC)**.

## 📖 Overview

This tool allows users to track Reddit activity directly from the terminal. It polls the Reddit JSON API to fetch posts from a specified subreddit, filters out duplicates that have already been seen during the session, and saves the data to a persistent local file.

It uses a custom **User-Agent** to comply with Reddit's API rules and avoid rate-limiting (HTTP 429/403 errors).

## ✨ Features

* **Customizable Targets:** Choose any subreddit to monitor via command-line arguments.
* **Sorting Options:** Support for `Hot` (default), `New`, and `Top` sorting methods.
* **Live Polling:** Automatically refreshes data at a user-defined interval (default: 60 seconds).
* **Data Persistence:** Saves posts to `subreddits/<subreddit_name>.json`. It reads existing files to prevent overwriting historical data.
* **Duplicate Detection:** Uses a `HashSet` in memory to ensure the same post isn't displayed or processed multiple times in a single run.

## 🛠️ Dependencies

This project relies on the following Rust crates:
* [clap](https://crates.io/crates/clap) - Command Line Argument Parser.
* [reqwest](https://crates.io/crates/reqwest) - HTTP Client for making API requests.
* [serde](https://crates.io/crates/serde) & [serde_json](https://crates.io/crates/serde_json) - Serialization and Deserialization of JSON data.
* [tokio](https://crates.io/crates/tokio) - Asynchronous runtime.

## 🚀 Usage

### Prerequisites
Ensure you have **Rust** and **Cargo** installed.
1.  Clone the repository.
2.  Create a `subreddits` directory in the root of the project (if not automatically handled) to store the output files:
    ```bash
    mkdir subreddits
    ```

### Running the Tool
Use `cargo run` followed by the arguments.
Arguments
Flag	             Short	               Description	                                          Default
<SUBREDDIT>	          N/A	Required. The name of the subreddit to monitor (e.g., rust, piracy).	N/A
--sort                -s	Sorting method: hot, new, or top.	                                    hot
--time	              -t	Interval in seconds between fetch requests.	                            60
**Syntax:**
```bash
cargo run -- <SUBREDDIT> [OPTIONS]
```

## Examples
### 1. Monitor the 'rust' subreddit (defaults to 'hot', 60s interval):
```
cargo run -- rust
```
### 2. Monitor 'foxes' sorted by 'new', checking every 10 seconds:
```
cargo run -- foxes -s new -t 10
```
### 3. Monitor 'technology' sorted by 'top', checking every 5 minutes (300s):
```
cargo run -- technology --sort top --time 300
```
## 📂 Output Data

The program stores data in the subreddits/ directory.

    File naming convention: <subreddit_name>.json

    If the file exists, the program loads it to ensure old posts are preserved and new posts are appended.

⚠️ specific Implementation Notes

    User Agent: The client uses a custom User-Agent string (rust-redditor) to prevent Reddit from blocking the requests as automated bot spam.

    Error Handling: If the client fails to build (e.g., TLS issues), the program will print an error to stderr and terminate immediately.
