// CLRF from RFC 9110
pub const SEPARATOR: &'static str = "\r\n";

// Request errors
pub const MALFORMED_REQUEST: &'static str = "Request is Malformed";
pub const ERROR_STATE: &'static str = "Request is in error state";

// Request line errors
pub const MALFORMED_REQUEST_LINE: &'static str = "Request line is Malformed";
pub const MALFORMED_HTTP_VERSION: &'static str = "HTTP Version is Malformed";
pub const MALFORMED_METHOD: &'static str = "Method is Malformed";

// Header errors
pub const MALFORMED_HEADER: &'static str = "HEADER is Malformed";
pub const MALFORMED_FIELD_NAME: &'static str = "Field value is Malformed";