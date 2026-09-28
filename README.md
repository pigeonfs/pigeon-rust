# Pigeon Rust SDK

Official Rust client for the [Pigeon](https://github.com/pigeonfs/pigeon) email API. Crate layout follows [resend-rust](https://github.com/resend/resend-rust): `Pigeon::new(api_key).emails.send(...)`.

## Install

```toml
[dependencies]
pigeon = { git = "https://github.com/pigeonfs/pigeon-rust" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```bash
cargo add --git https://github.com/pigeonfs/pigeon-rust pigeon
```

## Example

```rust
use pigeon::{Pigeon, SendEmailRequest};

#[tokio::main]
async fn main() -> Result<(), pigeon::Error> {
    let pigeon = Pigeon::new("pg_xxxx");
    let email = pigeon
        .emails
        .send(&SendEmailRequest {
            from: "Ada <ada@yourdomain.com>".into(),
            to: vec!["person@example.com".into()],
            subject: "Hello from Rust".into(),
            html: Some("<p>Hello</p>".into()),
            ..Default::default()
        })
        .await?;
    println!("Email {} has been sent", email.id);
    Ok(())
}
```

Set `PIGEON_BASE_URL` (default `http://localhost:4005`). Default TLS uses `rustls`.

See `examples/send.rs`.

## License

MIT
