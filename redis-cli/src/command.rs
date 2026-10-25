use std::collections::HashMap;

use crate::parser::RespValue;

type Handler = fn(&[Vec<u8>]) -> RespValue;

pub struct Registry {
    handlers: HashMap<String, Handler>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            handlers: HashMap::from([("PING".to_string(), ping as Handler)]),
        }
    }

    pub fn dispatch(&self, request: &RespValue) -> RespValue {
        let RespValue::Array(Some(items)) = request else {
            return RespValue::Error("ERR expected an array of bulk strings".into());
        };

        let parts: Option<Vec<Vec<u8>>> = items
            .iter()
            .map(|item| match item {
                RespValue::BulkString(Some(bytes)) => Some(bytes.clone()),
                _ => None,
            })
            .collect();

        let Some(parts) = parts else {
            return RespValue::Error("ERR expected an array of bulk strings".into());
        };

        let Some((name, args)) = parts.split_first() else {
            return RespValue::Error("ERR empty command".into());
        };

        let name = String::from_utf8_lossy(name).to_uppercase();

        match self.handlers.get(&name) {
            Some(handler) => handler(args),
            None => RespValue::Error(format!("ERR unknown command '{}'", name)),
        }
    }
}

fn ping(_args: &[Vec<u8>]) -> RespValue {
    RespValue::SimpleString("PONG".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_ping_returns_pong() {
        let input = RespValue::Array(Some(Vec::from([RespValue::BulkString(Some(
            b"PING".to_vec(),
        ))])));
        let registry = Registry::new();
        let output = registry.dispatch(&input);
        let expected = RespValue::SimpleString("PONG".to_string());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_dispatch_command_name_case_insensitive() {
        let input = RespValue::Array(Some(Vec::from([RespValue::BulkString(Some(
            b"PiNg".to_vec(),
        ))])));
        let registry = Registry::new();
        let output = registry.dispatch(&input);
        let expected = RespValue::SimpleString("PONG".to_string());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_dispatch_unknown_command_returns_error() {
        let input = RespValue::Array(Some(Vec::from([RespValue::BulkString(Some(
            b"UNKNOWN".to_vec(),
        ))])));
        let registry = Registry::new();
        let output = registry.dispatch(&input);
        let expected = RespValue::Error("ERR unknown command 'UNKNOWN'".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_dispatch_non_array_returns_error() {
        let input = RespValue::BulkString(Some(b"PING".to_vec()));
        let registry = Registry::new();
        let output = registry.dispatch(&input);
        let expected = RespValue::Error("ERR expected an array of bulk strings".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_dispatch_null_array_returns_error() {
        let input = RespValue::Array(None);
        let registry = Registry::new();
        let output = registry.dispatch(&input);
        let expected = RespValue::Error("ERR expected an array of bulk strings".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_dispatch_empty_array_returns_error() {
        let input = RespValue::Array(Some(Vec::new()));
        let registry = Registry::new();
        let output = registry.dispatch(&input);
        let expected = RespValue::Error("ERR empty command".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_dispatch_null_bulk_element_returns_error() {
        let input = RespValue::Array(Some(Vec::from([RespValue::BulkString(None)])));
        let registry = Registry::new();
        let output = registry.dispatch(&input);
        let expected = RespValue::Error("ERR expected an array of bulk strings".into());
        assert_eq!(output, expected);
    }
}
