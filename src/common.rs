use log::*;

pub const DEFAULT_AGENT_STR: &str = concat!(env!("CARGO_PKG_NAME"), "_rs-", env!("CARGO_PKG_VERSION"));

pub enum ClientRole {
    Caller,
    Callee,
    Publisher,
    Subscriber,
}
impl ClientRole {
    pub fn to_string(&self) -> String {
        String::from(
            match self {
                &ClientRole::Caller => "caller",
                &ClientRole::Callee => "callee",
                &ClientRole::Publisher => "publisher",
                &ClientRole::Subscriber => "subscriber",
            }
        )
    }
}

pub enum ServerRole {
    Router,
    Broker,
}
impl ServerRole {
    pub fn to_string(&self) -> String {
        String::from(
            match self {
                &ServerRole::Router => "router",
                &ServerRole::Broker => "broker",
            }
        )
    }
}

/// Returns whether a uri is valid or not (using strict rules)
pub fn is_valid_strict_uri<T: AsRef<str>>(in_uri: T) -> bool {
    let uri: &str = in_uri.as_ref();
    let mut num_chars_token: usize = 0;
    if uri.starts_with("wamp.") {
        warn!("URI '{}' cannot start with 'wamp'", uri);
        return false;
    }

    for (i, c) in uri.chars().enumerate() {
        if c == '.' {
            if num_chars_token == 0 {
                warn!("URI '{}' contains a zero length token ending @ index {}", uri, i);
                return false;
            }
            num_chars_token = 0;
        } else {
            num_chars_token += 1;
        }

        if c == '_' {
            continue;
        }

        if !c.is_lowercase() {
            warn!("URI '{}' contains a non lower case character @ index {}", uri, i);
            return false;
        }
        if !c.is_alphanumeric() {
            warn!("URI '{}' contains an invalid character @ index {}", uri, i);
            return false;
        }
    }
    
    return true;
}