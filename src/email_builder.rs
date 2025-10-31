use std::marker::PhantomData;

pub struct NoRecipient;
pub struct HasRecipient;
pub struct NoSubject;
pub struct HasSubject;

#[derive(Debug, PartialEq)]
pub struct EmailBuilder<R, S> {
    recipient: String,
    subject: String,
    body: Option<String>,
    _r_set: PhantomData<R>,
    _s_set: PhantomData<S>,
}

#[derive(Debug, PartialEq)]
pub struct Email {
    recipient: String,
    subject: String,
    body: Option<String>,
}

impl EmailBuilder<NoRecipient, NoSubject> {
    pub fn new() -> Self {
        Self {
            recipient: String::new(),
            subject: String::new(),
            body: None,
            _r_set: PhantomData,
            _s_set: PhantomData,
        }
    }
}

impl<S> EmailBuilder<NoRecipient, S> {
    pub fn recipient(self, email: String) -> EmailBuilder<HasRecipient, S> {
        EmailBuilder {
            recipient: email,
            subject: self.subject,
            body: self.body,
            _r_set: PhantomData,
            _s_set: PhantomData,
        }
    }
}

impl<R> EmailBuilder<R, NoSubject> {
    pub fn subject(self, text: String) -> EmailBuilder<R, HasSubject> {
        EmailBuilder {
            recipient: self.recipient,
            subject: text,
            body: self.body,
            _r_set: PhantomData,
            _s_set: PhantomData,
        }
    }
}

impl<R, S> EmailBuilder<R, S> {
    pub fn body(self, text: String) -> EmailBuilder<R, S> {
        EmailBuilder {
            recipient: self.recipient,
            subject: self.subject,
            body: Some(text),
            _r_set: PhantomData,
            _s_set: PhantomData,
        }
    }
}

impl EmailBuilder<HasRecipient, HasSubject> {
    pub fn send(self) -> Email {
        Email {
            recipient: self.recipient,
            subject: self.subject,
            body: self.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_with_body() {
        let email = EmailBuilder::new()
            .recipient("user@example.com".to_string())
            .subject("Hello".to_string())
            .body("Test message".to_string())
            .send();
        
        assert_eq!(email.recipient, "user@example.com");
        assert_eq!(email.subject, "Hello");
        assert_eq!(email.body, Some("Test message".to_string()));
    }
}
