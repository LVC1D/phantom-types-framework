use std::marker::PhantomData;

// 1. Marker types
pub struct NoUrl;
pub struct HasUrl;
pub struct NoMethod;
pub struct HasMethod;

// 2. The builder
pub struct RequestBuilder<U, M> {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    _url_state: PhantomData<U>,
    _method_state: PhantomData<M>,
}

// 3. The final Request type
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

// TODO: Implement the various impl blocks
impl RequestBuilder<NoUrl, NoMethod> {
    pub fn new() -> Self {
        Self {
            url: String::new(),
            method: String::new(),
            headers: Vec::new(),
            body: None,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    } 
}

impl<M> RequestBuilder<NoUrl, M> {
    pub fn set_url(self, url: String) -> RequestBuilder<HasUrl, M> {
        RequestBuilder {
            url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

impl<U> RequestBuilder<U, NoMethod> {
    pub fn set_method(self, method: String) -> RequestBuilder<U, HasMethod> {
        RequestBuilder {
            url: self.url,
            method,
            headers: self.headers,
            body: self.body,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

impl<U, M> RequestBuilder<U, M> {
    pub fn set_headers(mut self, headers: &[(String, String)]) -> RequestBuilder<U, M> {
        self.headers.extend_from_slice(headers);
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            _url_state: PhantomData,
            _method_state: PhantomData,

        }
    }

    pub fn set_body(self, body: String) -> RequestBuilder<U, M> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: Some(body),
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

impl RequestBuilder<HasUrl, HasMethod> {
    pub fn build(self) -> Request {
        Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_then_method() {
        let request = RequestBuilder::new()
            .set_url("https://api.github.com".to_string())
            .set_method("GET".to_string())
            .build();
        
        assert_eq!(request.url, "https://api.github.com");
        assert_eq!(request.method, "GET");
    }

    #[test]
    fn test_method_then_url() {
        let request = RequestBuilder::new()
            .set_method("POST".to_string())
            .set_url("https://api.github.com".to_string())
            .build();
        
        assert_eq!(request.url, "https://api.github.com");
        assert_eq!(request.method, "POST");
    }

    #[test]
    fn test_with_optional_fields() {
        let request = RequestBuilder::new()
            .set_url("https://api.github.com".to_string())
            .set_method("POST".to_string())
            .set_body("test body".to_string())
            .set_headers(&[("Authorization".to_string(), "Bearer token".to_string())])
            .build();
        
        assert_eq!(request.body, Some("test body".to_string()));
        assert_eq!(request.headers.len(), 1);
    }
}
