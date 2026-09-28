use pigeon::{Pigeon, SendEmailRequest};

#[tokio::main]
async fn main() -> Result<(), pigeon::Error> {
    let client = Pigeon::new(std::env::var("PIGEON_API_KEY").expect("PIGEON_API_KEY"));
    let email = client
        .emails
        .send(&SendEmailRequest {
            from: "Ada <ada@yourdomain.com>".into(),
            to: vec!["person@example.com".into()],
            subject: "Hello from Rust".into(),
            html: Some("<strong>it works!</strong>".into()),
            ..Default::default()
        })
        .await?;
    println!("{}", email.id);
    Ok(())
}
