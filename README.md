# lolclientapi_rs

Rust bindings for the locally running League of Legends client API (lcu) endpoints.
This crate handles authentication and deserialization, you can call methods directly on a LeagueClient instead of dealing with tokens and manual request handling.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
lolclientapi_rs = { version = "0.1.0", git = "https://github.com/Guido30/LoLClientApi-RS" }
```

Example usage

```rust
use lolclientapi_rs::blocking::LeagueClient;

fn main() {
    // Initialize the connection to the local lcu api (league client must be running)
    let mut client = LeagueClient::new();

    // Check if the api is responding
    let status = client.status()

    if status {
        // Call any implemented endpoint and get its serialized object back
        let result: client.get_lol_summoner_v1_current_summoner_account_and_summoner_ids().unwrap()
        let sum_id = result.summoner_id

        // Use the received data
        println!("Current summoner id: {}", sum_id)
    } else {
        // Connection can be re-established
        let _ = client.retry()
    }
}
```
