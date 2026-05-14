use std::collections::HashMap;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use anyhow::Context;

pub struct Config {
    pub endpoints: HashMap<String, Vec<Endpoint>>,
    pub addr: SocketAddr,
    pub handler_config: HandlerConfig,
}

#[derive(Clone)]
pub struct HandlerConfig {
    /// length of buffer in bytes to bail out when a header is not identified
    pub len_buf_bail: usize,
    /// timeout before bailing out for not identifying a header
    pub timeout_bail: Duration,
    pub request_policy: maht::RequestPolicy,
}

pub struct Endpoint {
    pub methods: Vec<maht::Method>,
    pub callback: CallbackFunc,
}

pub type CallbackFunc = Box<dyn Fn(Request) -> maht::Response + Send + Sync>;

pub struct Request {
    addr: SocketAddr,
    http: maht::Request,
    body: Option<Vec<u8>>
}

pub fn ignite(
    Config {
        endpoints,
        addr,
        handler_config,
    }: Config,
) -> anyhow::Result<()> {
    eprintln!("Midnight233's Another Web Server is starting for {addr}");
    let listener = TcpListener::bind(addr)?;
    let handlers = Arc::new(endpoints);
    let config = Arc::new(handler_config);
    loop {
        let (stream, addr) = listener.accept()?;
        let handlers = handlers.clone();
        let config = config.clone();
        thread::spawn(move || {
            eprintln!("{addr}");
            if let Err(err) = handle(stream, addr, handlers.clone(), config) {
                eprintln!("{}", err);
            }
        });
    }
}

fn handle(
    mut stream: TcpStream,
    addr: SocketAddr,
    endpoints: Arc<HashMap<String, Vec<Endpoint>>>,
    config: Arc<HandlerConfig>,
) -> anyhow::Result<()> {
    let HandlerConfig {
        len_buf_bail,
        timeout_bail,
        request_policy,
    } = config.as_ref();
    let mut buf = vec![0; 1024];
    let mut old_len = 0;
    let mut len = stream.read(&mut buf)?;
    let now = Instant::now();
    let len_req = loop {
        if let Some(len_rel) = identify_header(&buf[old_len..]) {
            break len_rel + old_len;
        }
        if &len == len_buf_bail {
            anyhow::bail!("did not find a header within toleratable buffer size")
        }
        if &now.elapsed() > timeout_bail {
            anyhow::bail!("did not find a header within toleratable interval")
        }
        if len == buf.len() {
            buf.resize(buf.len() * 2, 0);
        }
        let extra_len = stream.read(&mut buf[len..])?;
        old_len = len;
        len += extra_len;
    };
    let request = maht::Request::parse(&buf[..len_req], request_policy)?;
    Ok(())
}

fn identify_header(buf: &[u8]) -> Option<usize> {
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
        assert_eq!(identify_header(with_double_crlf), Some(with_double_crlf.len() - 3));
        let without_double_crlf = b"\
        hello, world\n, no, \ractually, hello, world\r\n\
        hello, world!!!\r\r\n\nlook at me!\
        ";
        assert_eq!(identify_header(without_double_crlf), None);
    }
}