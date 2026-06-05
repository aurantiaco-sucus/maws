use crate::cistr::AsCaseInsensitive;
use crate::CaseInsensitiveStr;

macro_rules! maht_status {
    ($(($name:ident, $name_upper:ident, $code16:literal, $msg_enc:literal))*) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum Status {
            $($name,)*
        }
        
        pub mod status_names {
            $(pub const $name_upper: &str = $msg_enc;)*
        }
        
        pub mod status_ordinals {
            $(pub const $name_upper: u16 = $code16;)*
        }
        
        impl Status {
            pub fn parse(value: &CaseInsensitiveStr) -> Option<Self> {
                $(if value == status_names::$name_upper.case_insensitive() {
                    Some(Status::$name)
                } else)* {
                    None
                }
            }
            
            pub fn parse_u16(value: u16) -> Option<Self> {
                match value {
                    $(status_ordinals::$name_upper => Some(Self::$name),)*
                    _ => None
                }
            }
            
            pub fn as_str(&self) -> &str {
                match self {
                    $(Status::$name => status_names::$name_upper,)*
                }
            }
            
            pub fn as_u16(&self) -> u16 {
                match self {
                    $(Status::$name => status_ordinals::$name_upper,)*
                }
            }
        }
    };
}

maht_status! {
    (Continue100, CONTINUE_100, 100, "100 Continue")
    (SwitchingProtocols101, SWITCHING_PROTOCOLS_101, 101, "101 Switching Protocols")
    (Processing102, PROCESSING_102, 102, "102 Processing")
    (EarlyHints103, EARLY_HINTS_103, 103, "103 Early Hints")
    (Ok200, OK_200, 200, "200 OK")
    (Created201, CREATED_201, 201, "201 Created")
    (Accepted202, ACCEPTED_202, 202, "202 Accepted")
    (NonAuthoritativeInformation203, NON_AUTHORITATIVE_INFORMATION_203, 203, "203 Non-Authoritative Information")
    (NoContent204, NO_CONTENT_204, 204, "204 No Content")
    (ResetContent205, RESET_CONTENT_205, 205, "205 Reset Content")
    (PartialContent206, PARTIAL_CONTENT_206, 206, "206 Partial Content")
    (MultiStatus207, MULTI_STATUS_207, 207, "207 Multi-Status")
    (AlreadyReported208, ALREADY_REPORTED_208, 208, "208 Already Reported")
    (ImUsed226, IM_USED_226, 226, "226 IM Used")
    (MultipleChoices300, MULTIPLE_CHOICES_300, 300, "300 Multiple Choices")
    (MovedPermanently301, MOVED_PERMANENTLY_301, 301, "301 Moved Permanently")
    (Found302, FOUND_302, 302, "302 Found")
    (SeeOther303, SEE_OTHER_303, 303, "303 See Other")
    (NotModified304, NOT_MODIFIED_304, 304, "304 Not Modified")
    (UseProxy305, USE_PROXY_305, 305, "305 Use Proxy")
    (TemporaryRedirect307, TEMPORARY_REDIRECT_307, 307, "307 Temporary Redirect")
    (PermanentRedirect308, PERMANENT_REDIRECT_308, 308, "308 Permanent Redirect")
    (BadRequest400, BAD_REQUEST_400, 400, "400 Bad Request")
    (Unauthorized401, UNAUTHORIZED_401, 401, "401 Unauthorized")
    (PaymentRequired402, PAYMENT_REQUIRED_402, 402, "402 Payment Required")
    (Forbidden403, FORBIDDEN_403, 403, "403 Forbidden")
    (NotFound404, NOT_FOUND_404, 404, "404 Not Found")
    (MethodNotAllowed405, METHOD_NOT_ALLOWED_405, 405, "405 Method Not Allowed")
    (NotAcceptable406, NOT_ACCEPTABLE_406, 406, "406 Not Acceptable")
    (ProxyAuthenticationRequired407, PROXY_AUTHENTICATION_REQUIRED_407, 407, "407 Proxy Authentication Required")
    (RequestTimeout408, REQUEST_TIMEOUT_408, 408, "408 Request Timeout")
    (Conflict409, CONFLICT_409, 409, "409 Conflict")
    (Gone410, GONE_410, 410, "410 Gone")
    (LengthRequired411, LENGTH_REQUIRED_411, 411, "411 Length Required")
    (PreconditionFailed412, PRECONDITION_FAILED_412, 412, "412 Precondition Failed")
    (ContentTooLarge413, CONTENT_TOO_LARGE_413, 413, "413 Content Too Large")
    (UriTooLong414, URI_TOO_LONG_414, 414, "414 URI Too Long")
    (UnsupportedMediaType415, UNSUPPORTED_MEDIA_TYPE_415, 415, "415 Unsupported Media Type")
    (RangeNotSatisfiable416, RANGE_NOT_SATISFIABLE_416, 416, "416 Range Not Satisfiable")
    (ExpectationFailed417, EXPECTATION_FAILED_417, 417, "417 Expectation Failed")
    (ImATeapot418, IM_A_TEAPOT_418, 418, "418 I'm a teapot")
    (MisdirectedRequest421, MISDIRECTED_REQUEST_421, 421, "421 Misdirected Request")
    (UnprocessableEntity422, UNPROCESSABLE_ENTITY_422, 422, "422 Unprocessable Entity")
    (Locked423, LOCKED_423, 423, "423 Locked")
    (FailedDependency424, FAILED_DEPENDENCY_424, 424, "424 Failed Dependency")
    (TooEarly425, TOO_EARLY_425, 425, "425 Too Early")
    (UpgradeRequired426, UPGRADE_REQUIRED_426, 426, "426 Upgrade Required")
    (PreconditionRequired428, PRECONDITION_REQUIRED_428, 428, "428 Precondition Required")
    (TooManyRequests429, TOO_MANY_REQUESTS_429, 429, "429 Too Many Requests")
    (RequestHeaderFieldsTooLarge431, REQUEST_HEADER_FIELDS_TOO_LARGE_431, 431, "431 Request Header Fields Too Large")
    (UnavailableForLegalReasons451, UNAVAILABLE_FOR_LEGAL_REASONS_451, 451, "451 Unavailable For Legal Reasons")
    (InternalServerError500, INTERNAL_SERVER_ERROR_500, 500, "500 Internal Server Error")
    (NotImplemented501, NOT_IMPLEMENTED_501, 501, "501 Not Implemented")
    (BadGateway502, BAD_GATEWAY_502, 502, "502 Bad Gateway")
    (ServiceUnavailable503, SERVICE_UNAVAILABLE_503, 503, "503 Service Unavailable")
    (GatewayTimeout504, GATEWAY_TIMEOUT_504, 504, "504 Gateway Timeout")
    (HttpVersionNotSupported505, HTTP_VERSION_NOT_SUPPORTED_505, 505, "505 HTTP Version Not Supported")
    (VariantAlsoNegotiates506, VARIANT_ALSO_NEGOTIATES_506, 506, "506 Variant Also Negotiates")
    (InsufficientStorage507, INSUFFICIENT_STORAGE_507, 507, "507 Insufficient Storage")
    (LoopDetected508, LOOP_DETECTED_508, 508, "508 Loop Detected")
    (NotExtended510, NOT_EXTENDED_510, 510, "510 Not Extended")
    (NetworkAuthenticationRequired511, NETWORK_AUTHENTICATION_REQUIRED_511, 511, "511 Network Authentication Required")
}
