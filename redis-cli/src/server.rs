use crate::{
    command::Registry,
    encoder::Encoder,
    error::RedisError,
    parser::{Parser, RespValue},
};

pub fn process(buf: &mut Vec<u8>, registry: &Registry) -> Result<Vec<u8>, RedisError> {
    let mut out: Vec<u8> = Vec::new();
    while !buf.is_empty() {
        let mut parser = Parser::new(buf.clone());

        match parser.parse() {
            Ok(request) => {
                let reply = registry.dispatch(&request);
                out.extend_from_slice(&Encoder::encode(&reply)?);
                buf.drain(..parser.consumed());
            }
            Err(RedisError::Incomplete) => break,
            Err(e) => {
                let reply = RespValue::Error(format!("ERR Protocol error: {}", e));
                out.extend_from_slice(&Encoder::encode(&reply)?);
                buf.clear();
                break;
            }
        }
    }

    Ok(out)
}
