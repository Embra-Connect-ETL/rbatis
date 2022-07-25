use crate::arguments::{PgArgumentBuffer, PgArguments};
use crate::type_info::PgType::Json;
use crate::type_info::PgTypeInfo;
use crate::types::Oid;
use rbs::Value;
use std::mem;

pub enum IsNull {
    No,
    Yes,
}
pub trait Encode {
    fn type_info(&self) -> PgTypeInfo;
    fn encode(self, arg: &mut PgArgumentBuffer) -> IsNull;
}

impl From<Vec<Value>> for PgArguments {
    fn from(args: Vec<Value>) -> Self {
        let mut arg = PgArguments {
            types: Vec::with_capacity(args.len()),
            buffer: PgArgumentBuffer::default(),
        };
        for x in args {
            arg.add(x);
        }
        arg
    }
}

impl Encode for Value {
    fn type_info(&self) -> PgTypeInfo {
        match self {
            Value::Null => PgTypeInfo::with_name("NULL"),
            Value::Bool(_) => {
                todo!()
            }
            Value::I32(_) => {
                todo!()
            }
            Value::I64(_) => {
                todo!()
            }
            Value::U32(_) => {
                todo!()
            }
            Value::U64(_) => {
                todo!()
            }
            Value::F32(_) => {
                todo!()
            }
            Value::F64(_) => {
                todo!()
            }
            Value::String(_) => PgTypeInfo::VARCHAR,
            Value::Binary(_) => {
                todo!()
            }
            Value::Array(_) => {
                todo!()
            }
            Value::Map(_) => {
                todo!()
            }
            Value::Ext(type_name, _) => {
                match *type_name {
                    "uuid" => PgTypeInfo::UUID,
                    //decimal = 12345678
                    "decimal" => PgTypeInfo::NUMERIC,
                    //Date = "1993-02-06"
                    "date" => PgTypeInfo::DATE,
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "time" => PgTypeInfo::TIME,
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "timestamp" => PgTypeInfo::TIMESTAMP,
                    "datetime" => PgTypeInfo::TIMESTAMP,
                    "json" => PgTypeInfo::JSON,
                    "oid" => PgTypeInfo::OID,

                    _ => PgTypeInfo::UNKNOWN,
                }
            }
        }
    }

    fn encode(self, arg: &mut PgArgumentBuffer) -> IsNull {
        match self {
            Value::Null => IsNull::Yes,
            Value::Bool(v) => todo!(),
            Value::I32(v) => {
                todo!()
            }
            Value::I64(v) => {
                todo!()
            }
            Value::U32(v) => {
                todo!()
            }
            Value::U64(v) => {
                todo!()
            }
            Value::F32(v) => {
                todo!()
            }
            Value::F64(v) => {
                todo!()
            }
            Value::String(v) => {
                //default -> string
                v.encode(arg)
            }
            Value::Binary(v) => {
                todo!()
            }
            Value::Array(v) => {
                todo!()
            }
            Value::Map(v) => {
                todo!()
            }
            Value::Ext(type_name, v) => {
                match type_name {
                    "uuid" => {
                        todo!()
                    }
                    //decimal = 12345678
                    "decimal" => {
                        todo!()
                    }
                    //Date = "1993-02-06"
                    "date" => {
                        todo!()
                    }
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "time" => {
                        todo!()
                    }
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "timestamp" => {
                        todo!()
                    }
                    "datetime" => {
                        todo!()
                    }
                    "json" => crate::types::json::Json::from(v.into_string().unwrap_or_default())
                        .encode(arg),

                    "oid" => Oid::from(v.as_u64().unwrap_or_default() as u32).encode_by_ref(arg),

                    _ => IsNull::Yes,
                }
            }
        }
    }
}

impl Encode for String {
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::VARCHAR
    }

    fn encode(self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(self.into_bytes());
        IsNull::No
    }
}
