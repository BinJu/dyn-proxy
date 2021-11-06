mod filter;

use hyper::{Body, Request, Response, header::HeaderValue};
use async_trait::async_trait;

use std::error::Error;

use crate::server;

/// AgentService behaves as an agent. It accesses the target url. Before it returns the response to
/// you, it will remove some of the div tags that you specified. It can help you to filter out some
/// div's that you are reluctant to see.
///
/// We specially have the `mock_response` and the `last_response` for UT. Because we can not mock
/// reqwest easily. And the response is in Response<Body>, There is probably no way to get value
/// out from Body.
///
#[derive(Debug, Clone)]
pub struct AgentService{
    service_name: String,
    base_url: String,
    port: u16,
    div_props: Vec<String>,
    #[cfg(test)]
    mock_response: String,
    #[cfg(test)]
    last_response: String,
}

impl AgentService {
    #[cfg(test)]
    pub fn new(service_name: String, base_url: String, port: u16, div_props: Vec<String>) -> AgentService {
        AgentService{service_name, base_url, port, div_props, mock_response: "".to_owned(), last_response: "".to_owned()}
    }
    #[cfg(not(test))]
    pub fn new(service_name: String, base_url: String, port: u16, div_props: Vec<String>) -> AgentService {
        AgentService{service_name, base_url, port, div_props}
    }
}

#[async_trait]
impl server::Service for AgentService {
    async fn handle(&mut self, req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
        let request_url = format!("{}{}", self.base_url, req.uri());
        let filter = filter::ContentFilter::new(self.div_props.clone());
        let rendered_text = filter.filter(self.agent(request_url).await?);
        #[cfg(test)]
        { self.last_response = rendered_text.clone(); }
        let is_html = rendered_text.contains("<html");
        let mut response = Response::new(rendered_text.into());

        if  is_html {
            let headers = response.headers_mut();
            headers.insert("Content-Type", HeaderValue::from_static("text/html; charset=UTF-8"));
        }
        Ok(response)
    }

    #[cfg(not(test))]
    async fn agent(&self, url: String) -> Result<String, Box<dyn Error + Send + Sync>> {
       Ok(reqwest::get(url).await?.text().await?) 
    }

    #[cfg(test)]
    async fn agent(&self, _url: String) -> Result<String, Box<dyn Error + Send + Sync>> {
       Ok(self.mock_response.clone())
    }
    fn port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
mod tests {

    use super::*; 
    use crate::server::Service;

    #[tokio::test] async fn test_agent_service_receive_content() {
        let req = Request::new(Body::empty());
        let mut service = AgentService::new("test".into(), "http://dummy".into(), 3000, Vec::new());
        service.mock_response = "hello world".to_owned();
        let _ = service.handle(req).await.unwrap();
        let text = service.last_response;
        assert_eq!(text, "hello world");
    }

    #[tokio::test] async fn test_agent_service_html_header() {
        let req = Request::new(Body::empty());
        let mut service = AgentService::new("test".into(), "http://dummy".into(), 3000, Vec::new());
        service.mock_response = "<html><head></head><body>hello world</body></html>".to_owned();
        let response = service.handle(req).await.unwrap();
        let text = service.last_response;
        assert_eq!(text, "<html><head></head><body>hello world</body></html>");
        let headers = response.headers();
        let content_type = headers.get("Content-Type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/html; charset=UTF-8");
    }

    #[tokio::test] async fn test_agent_service_non_html_header() {
        let req = Request::new(Body::empty());
        let mut service = AgentService::new("test".into(), "http://dummy".into(), 3000, Vec::new());
        service.mock_response = "this is a non-html source".to_owned();
        let response = service.handle(req).await.unwrap();
        let text = service.last_response;
        assert_eq!(text, "this is a non-html source");
        let headers = response.headers();
        let content_type = headers.get("Content-Type");
        assert!(content_type.is_none());
    }
}
