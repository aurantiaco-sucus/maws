mod util;
pub mod ext;
pub mod route;
pub mod handler;

use crate::util::StreamBuffer;
use anyhow::Context;
use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

pub use maws_http as http;
use maws_http::RequestTarget;
use crate::route::Routes;

pub struct Config {
    pub routes: route::Routes,
    pub addr: SocketAddr,
    pub handler_config: HandlerConfig,
}

pub type EndpointMap = HashMap<http::ByteString<true>, BTreeMap<http::Method, EndpointFunc>>;

#[derive(Clone)]
pub struct HandlerConfig {
    /// length of buffer in bytes to bail out when a header is not identified
    pub len_buf_bail: usize,
    /// timeout before bailing out for not identifying a header
    pub timeout_header: Duration,
    /// policy for `http`'s request parsing functionality
    pub request_policy: http::RequestPolicy,
    /// factor of last request size (including body) that is kept for next request on the same
    /// TCP connection
    pub factor_leniency: usize,
    /// timeout before bailing out for unable to finish writing a part of the response
    pub timeout_write: Duration,
}

impl Default for HandlerConfig {
    fn default() -> Self {
        Self {
            len_buf_bail: 4096,
            timeout_header: Duration::from_secs(10),
            request_policy: http::RequestPolicy::default(),
            factor_leniency: 4,
            timeout_write: Duration::from_secs(10),
        }
    }
}

pub type EndpointFunc = Box<dyn Fn(Request) -> Response + Send + Sync + 'static>;

pub fn endpoint_func(func: impl Fn(Request) -> Response + Send + Sync + 'static) -> EndpointFunc {
    Box::new(func)
}

pub fn endpoint(method: http::Method, func: impl Fn(Request) -> Response + Send + Sync + 'static) -> (http::Method, EndpointFunc) {
    (method, endpoint_func(func))
}

pub struct Request {
    pub addr: SocketAddr,
    pub http: http::Request,
    pub body: Option<Vec<u8>>,
}

pub struct Response {
    pub http: http::Response,
    pub body: Option<Vec<u8>>,
}

pub fn ignite(
    Config {
        routes,
        addr,
        handler_config,
    }: Config,
) -> anyhow::Result<()> {
    eprintln!("Midnight233's Another Web Server is starting for {addr}");
    let listener = TcpListener::bind(addr)?;
    let routes = Arc::new(routes);
    let config = Arc::new(handler_config);
    loop {
        let (stream, addr) = listener.accept()?;
        let mut buf = StreamBuffer::new(stream, config.len_buf_bail);
        let routes = routes.clone();
        let config = config.clone();
        thread::spawn(move || {
            eprintln!("{addr}");
            loop {
                if let Err(err) = handle(&mut buf, addr, &routes, &config) {
                    eprintln!("{err}");
                    return;
                }
            }
        });
    }
}

fn handle(
    buf: &mut StreamBuffer<TcpStream>,
    addr: SocketAddr,
    routes: &Routes,
    HandlerConfig {
        len_buf_bail,
        timeout_header,
        request_policy,
        factor_leniency,
        timeout_write,
    }: &HandlerConfig,
) -> anyhow::Result<()> {
    buf.inner_mut().set_write_timeout(Some(*timeout_write))
        .context("unable to setup writing timeout")?;
    let now = Instant::now();
    let len_req = loop {
        let i_sniff = buf.len_old.saturating_sub(4);
        if let Some(len_rel) = identify_header(&buf.buf_eff()[i_sniff..]) {
            break i_sniff + len_rel;
        }
        if &buf.len == len_buf_bail {
            anyhow::bail!("did not find a header within toleratable buffer size")
        }
        if &now.elapsed() > timeout_header {
            anyhow::bail!("did not find a header within toleratable interval")
        }
        buf.read()
            .context("error reading the stream during header detection")?;
    };
    let request = String::from_utf8(buf.buf_eff()[..(len_req - 4)].to_vec())
        .context("request is not valid UTF-8")?;
    let request = http::Request::parse(&request, request_policy)?;
    let target = match &request.target {
        RequestTarget::Origin(path) => path,
        _ => anyhow::bail!("invalid request origin: {:?}", request.target)
    };
    let (handler, args) = routes.lookup(target)
        .with_context(|| format!("handler not found for path: {target}"))?;
    let endpoint = handler.get(&request.target)
        .and_then(|x| x.get(&request.method))
        .unwrap_or(default_endpoint);
    let len_body = request.content_length(request_policy)?.unwrap_or(0) as usize;
    let body = if len_body > 0 {
        while buf.len < len_req + len_body {
            buf.read()
                .context("error reading the stream to get request body")?;
        }
        Some(buf.buf_eff()[len_req..(len_req + len_body)].to_vec())
    } else {
        None
    };
    let request = Request {
        addr, http: request, body
    };
    buf.fit_factor(*factor_leniency);
    buf.drop_earliest(len_req + len_body);
    let response = endpoint(request);
    let http_resp: Vec<u8> = (&response.http).into();
    buf.inner_mut().write_all(&http_resp)
        .context("unable to write all bytes of response header")?;
    if let Some(body) = response.body {
        buf.inner_mut().write_all(b"\r\n")
            .context("unable to write response body")?;
        buf.inner_mut().write_all(&body)
            .context("unable to write response body")?;
    }
    Ok(())
}

fn identify_header(buf: &[u8]) -> Option<usize> {
    if buf.len() < 4 {
        return None
    }
    const DOUBLE_CRLF: &[u8] = b"\r\n\r\n";
    for i in 0..(buf.len() - 3) {
        if &buf[i..i + 4] == DOUBLE_CRLF {
            return Some(i + 4);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_identify_header() {
        let with_double_crlf = b"\
        hello, world\n, no, \ractually, hello, world\r\n\
        hello, world!!!\r\n\r\n!!!\
        ";
        assert_eq!(
            identify_header(with_double_crlf),
            Some(with_double_crlf.len() - 3)
        );
        let without_double_crlf = b"\
        hello, world\n, no, \ractually, hello, world\r\n\
        hello, world!!!\r\r\n\nlook at me!\
        ";
        assert_eq!(identify_header(without_double_crlf), None);
    }
}
