use crate::error::Error;
use puniyu_command::{Arg, ArgMode, ArgType, ArgValue};
use std::collections::HashMap;

/// 根据参数定义解析 token 列表为参数值 map。
pub(crate) fn parse_args(
    tokens: &[String],
    arg_defs: &[Arg<'_>],
) -> Result<HashMap<String, ArgValue>, Error> {
    let mut result = HashMap::new();
    let mut positional_index = 0;
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        if let Some(name) = token.strip_prefix("--") {
            let name = name.to_string();
            let def = arg_defs
                .iter()
                .find(|a| a.mode == ArgMode::Named && a.name.as_ref() == name)
                .ok_or(Error::UnknownArgument { arg_name: name.clone() })?;

            let value = if def.arg_type == ArgType::Bool {
                // Bool 命名参数：下一个 token 存在且不是 -- 开头时消费，否则默认 true
                if i + 1 < tokens.len() && !tokens[i + 1].starts_with("--") {
                    i += 1;
                    parse_value(&tokens[i], &def.arg_type, &name)?
                } else {
                    ArgValue::Bool(true)
                }
            } else {
                if i + 1 >= tokens.len() {
                    return Err(Error::MissingRequired { arg_name: name });
                }
                i += 1;
                parse_value(&tokens[i], &def.arg_type, &name)?
            };

            result.insert(name, value);
        } else {
            let def = arg_defs
                .iter()
                .filter(|a| a.mode == ArgMode::Positional)
                .nth(positional_index)
                .ok_or(Error::TooManyValues { arg_name: token.clone() })?;

            let name = def.name.to_string();
            let value = parse_value(token, &def.arg_type, &name)?;
            result.insert(name, value);
            positional_index += 1;
        }

        i += 1;
    }

    for def in arg_defs {
        if def.required && !result.contains_key(def.name.as_ref()) {
            return Err(Error::MissingRequired { arg_name: def.name.to_string() });
        }
    }

    Ok(result)
}

fn parse_value(raw: &str, arg_type: &ArgType, arg_name: &str) -> Result<ArgValue, Error> {
    match arg_type {
        ArgType::String => Ok(ArgValue::String(raw.to_string())),
        ArgType::Int => raw.parse::<i64>().map(ArgValue::Int).map_err(|_| Error::InvalidValue {
            arg_name: arg_name.to_string(),
            expected_type: "integer".to_string(),
            raw_value: raw.to_string(),
        }),
        ArgType::Float => {
            raw.parse::<f64>().map(ArgValue::Float).map_err(|_| Error::InvalidValue {
                arg_name: arg_name.to_string(),
                expected_type: "float".to_string(),
                raw_value: raw.to_string(),
            })
        }
        ArgType::Bool => match raw {
            "true" | "1" | "yes" => Ok(ArgValue::Bool(true)),
            "false" | "0" | "no" => Ok(ArgValue::Bool(false)),
            _ => Err(Error::InvalidValue {
                arg_name: arg_name.to_string(),
                expected_type: "boolean".to_string(),
                raw_value: raw.to_string(),
            }),
        },
    }
}
