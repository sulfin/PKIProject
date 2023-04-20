use actix_multipart::form::json::JsonFieldError::ContentType;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


async fn mail(custmail:&str, otp:&str) -> Result<(), lettre::Error> {
    let mailcreds=parsemail();
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse()?)
        .to(custmail.parse()?)
        .subject("OTP code")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("your OTP code is:",to_string(otp)))?;

    let creds = Credentials::new(mailcreds[nom].to_owned(), mailcreds[mdp].to_owned());

// Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

// Send the email
    mailer.send(&email)?;

    Ok(())
}
fn parsemail() {
    let mailcred = {
        // Load the first file into a string.
        let text = std::fs::read_to_string("./mailcred.json").unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };
}
