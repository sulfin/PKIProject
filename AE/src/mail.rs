use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::{to_string, Value};


async fn mail(custmail:&str, otp:&str) {
    let mailcred = {
        // Load the first file into a string.
        let text = std::fs::read_to_string("../mailcred.json").unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .to(custmail.parse().unwrap())
        .subject("OTP code")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(otp)).unwrap();
    //get credentials from mailcred.json
    let creds = Credentials::new(
        mailcred["username"].as_str().unwrap().to_string(),
        mailcred["password"].as_str().unwrap().to_string(),
    );


    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

// Send the email
    mailer.send(&email).unwrap();
}
