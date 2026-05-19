use crate::{const_byte_str, ByteStr};

macro_rules! maht_status {
    ($(($name:ident, $code16:literal, $msg_enc:literal))*) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum Status {
            $($name,)*
        }
        $(const_byte_str!($name = i:$msg_enc);)*
        impl TryFrom<&'static ByteStr<false>> for Status {
            type Error = &'static str;
            fn try_from(value: &'static ByteStr<false>) -> Result<Self, Self::Error> {
                $(if value == $name {
                    Ok(Status::$name)
                } else)* {
                    Err("unable to parse the status code")
                }
            }
        }
        impl TryFrom<u16> for Status {
            type Error = &'static str;
            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    $($code16 => Ok(Status::$name),)*
                    _ => Err("unable to parse the status code")
                }
            }
        }
        impl From<Status> for &'static ByteStr<false> {
            fn from(value: Status) -> Self {
                match value {
                    $(Status::$name => $name,)*
                }
            }
        }
        impl From<Status> for u16 {
            fn from(value: Status) -> Self {
                match value {
                    $(Status::$name => $code16,)*
                }
            }
        }
    };
}

maht_status! {
    (Continue100, 100, b"100 Continue")
    (SwitchingProtocols101, 101, b"101 Switching Protocols")
    (Processing102, 102, b"102 Processing")
    (EarlyHints103, 103, b"103 Early Hints")
    (Ok200, 200, b"200 OK")
    (Created201, 201, b"201 Created")
    (Accepted202, 202, b"202 Accepted")
    (NonAuthoritativeInformation203, 203, b"203 Non-Authoritative Information")
    (NoContent204, 204, b"204 No Content")
    (ResetContent205, 205, b"205 Reset Content")
    (PartialContent206, 206, b"206 Partial Content")
    (MultiStatus207, 207, b"207 Multi-Status")
    (AlreadyReported208, 208, b"208 Already Reported")
    (ImUsed226, 226, b"226 IM Used")
    (MultipleChoices300, 300, b"300 Multiple Choices")
    (MovedPermanently301, 301, b"301 Moved Permanently")
    (Found302, 302, b"302 Found")
    (SeeOther303, 303, b"303 See Other")
    (NotModified304, 304, b"304 Not Modified")
    (UseProxy305, 305, b"305 Use Proxy")
    (TemporaryRedirect307, 307, b"307 Temporary Redirect")
    (PermanentRedirect308, 308, b"308 Permanent Redirect")
    (BadRequest400, 400, b"400 Bad Request")
    (Unauthorized401, 401, b"401 Unauthorized")
    (PaymentRequired402, 402, b"402 Payment Required")
    (Forbidden403, 403, b"403 Forbidden")
    (NotFound404, 404, b"404 Not Found")
    (MethodNotAllowed405, 405, b"405 Method Not Allowed")
    (NotAcceptable406, 406, b"406 Not Acceptable")
    (ProxyAuthenticationRequired407, 407, b"407 Proxy Authentication Required")
    (RequestTimeout408, 408, b"408 Request Timeout")
    (Conflict409, 409, b"409 Conflict")
    (Gone410, 410, b"410 Gone")
    (LengthRequired411, 411, b"411 Length Required")
    (PreconditionFailed412, 412, b"412 Precondition Failed")
    (ContentTooLarge413, 413, b"413 Content Too Large")
    (UriTooLong414, 414, b"414 URI Too Long")
    (UnsupportedMediaType415, 415, b"415 Unsupported Media Type")
    (RangeNotSatisfiable416, 416, b"416 Range Not Satisfiable")
    (ExpectationFailed417, 417, b"417 Expectation Failed")
    (ImATeapot418, 418, b"418 I'm a teapot")
    (MisdirectedRequest421, 421, b"421 Misdirected Request")
    (UnprocessableEntity422, 422, b"422 Unprocessable Entity")
    (Locked423, 423, b"423 Locked")
    (FailedDependency424, 424, b"424 Failed Dependency")
    (TooEarly425, 425, b"425 Too Early")
    (UpgradeRequired426, 426, b"426 Upgrade Required")
    (PreconditionRequired428, 428, b"428 Precondition Required")
    (TooManyRequests429, 429, b"429 Too Many Requests")
    (RequestHeaderFieldsTooLarge431, 431, b"431 Request Header Fields Too Large")
    (UnavailableForLegalReasons451, 451, b"451 Unavailable For Legal Reasons")
    (InternalServerError500, 500, b"500 Internal Server Error")
    (NotImplemented501, 501, b"501 Not Implemented")
    (BadGateway502, 502, b"502 Bad Gateway")
    (ServiceUnavailable503, 503, b"503 Service Unavailable")
    (GatewayTimeout504, 504, b"504 Gateway Timeout")
    (HttpVersionNotSupported505, 505, b"505 HTTP Version Not Supported")
    (VariantAlsoNegotiates506, 506, b"506 Variant Also Negotiates")
    (InsufficientStorage507, 507, b"507 Insufficient Storage")
    (LoopDetected508, 508, b"508 Loop Detected")
    (NotExtended510, 510, b"510 Not Extended")
    (NetworkAuthenticationRequired511, 511, b"511 Network Authentication Required")
}