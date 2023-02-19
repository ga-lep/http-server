use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>
}


#[derive(PartialEq, Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value<'buffer>> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(buffer: &'buffer str) -> Self {
        let mut data = HashMap::new();

        for iterator in buffer.split('&') {
            let mut key = iterator;
            let mut value= "";
            if let Some(i) = iterator.find('=') {
                key = &iterator[..i];
                value = &iterator[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value|
                    match existing {
                        Value::Single(existing_value) => {
                            *existing = Value::Multiple(vec![existing_value, value])
                        }
                        Value::Multiple(vec) => vec.push(value)
                    })
                .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::{QueryString, QueryStringValue};

    #[test]
    fn test_single_simple() {
        let result = QueryString::from("a=1");

        assert!(result.get("a").is_some());
        assert_eq!(result.get("a").unwrap(), &QueryStringValue::Single("1"));
    }

    #[test]
    fn test_multiple_simple() {
        let result = QueryString::from("a=1&a=2&a=3");

        assert!(result.get("a").is_some());
        let list = vec!["1", "2", "3"];
        assert_eq!(result.get("a").unwrap(), &QueryStringValue::Multiple(list));
    }

    #[test]
    fn test_complex() {
        let result = QueryString::from("a=1&b=2&c&d=&e===&d=7&d=abc");

        assert!(result.get("a").is_some());
        assert_eq!(result.get("a").unwrap(), &QueryStringValue::Single("1"));

        assert!(result.get("b").is_some());
        assert_eq!(result.get("b").unwrap(), &QueryStringValue::Single("2"));

        assert!(result.get("c").is_some());
        assert_eq!(result.get("c").unwrap(), &QueryStringValue::Single(""));

        let list = vec!["", "7", "abc"];
        assert!(result.get("d").is_some());
        assert_eq!(result.get("d").unwrap(), &QueryStringValue::Multiple(list));

        assert!(result.get("e").is_some());
        assert_eq!(result.get("e").unwrap(), &QueryStringValue::Single("=="));
    }

}
