#[cfg(test)]
mod tests {
    use apple_dev_apis::XcodeSession;
    use icloud_auth::*;

    #[tokio::test]
    async fn xcsession_test() {
        println!("gsa auth test");

        let email = std::env::var("apple_email").unwrap_or_else(|_| {
            println!("Enter Apple email: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        });

        let password = std::env::var("apple_password").unwrap_or_else(|_| {
            println!("Enter Apple password: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        });

        let appleid_closure = move || (email.clone(), password.clone());
        // ask console for 2fa code, make sure it is only 6 digits, no extra characters
        let tfa_closure = || {
            println!("Enter 2FA code: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        };
        let acc = AppleAccount::login(appleid_closure, tfa_closure, AnisetteConfiguration::default()).await;
        let session = XcodeSession::with(&acc.unwrap());
    }
}
