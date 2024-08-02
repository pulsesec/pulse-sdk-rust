<h1 align="center"><a href="https://www.pulsesecurity.org/">Pulse Security</a></h1>
<p align="center">
<img src="https://avatars.githubusercontent.com/u/161549711?s=200&v=4"/>
</p>
<h1 align="center">Rust SDK</h1>

## Installation

```sh
cargo add pulsesecurity
```

```toml
pulsesecurity = "0.1.2"
```

## Verification

```rs
use pulsesecurity::{Pulse, PulseError};

#[tokio::main]
async fn main() {
    let pulse = Pulse::new("SITE_KEY".to_string(), "SECRET_KEY".to_string());
    let token = "REQUEST_TOKEN".to_string();

    match pulse.classify(token).await {
        Ok(is_bot) => {
            println!("Result: {}", is_bot);
        }
        Err(err) => match err {
            PulseError::TokenNotFoundError(_) => {
                println!("Token not found");
            }
            PulseError::TokenUsedError(_) => {
                println!("Token already used");
            }
            PulseError::TokenExpiredError(_) => {
                println!("Token expired");
            }
            _ => {
                println!("Unknown error")
            }
        },
    }
}
```
