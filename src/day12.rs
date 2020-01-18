use crate::input::get_contents;
use serde_json::Value;

pub fn run() {
    let json = get_contents("day12");

    println!("part2: {}", number_sum(&json));
}

fn number_sum(json: &str) -> i64 {
    let v: Value = serde_json::from_str(json).unwrap();
    sum_value(&v)
}

fn sum_value(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(a) => a.iter().map(sum_value).sum(),
        Value::Object(o) => {
            if o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(sum_value).sum()
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(number_sum("[1,2,3]"), 6);
        assert_eq!(number_sum(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(number_sum("[[[3]]]"), 3);
        assert_eq!(number_sum(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(number_sum(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(number_sum(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(number_sum("[]"), 0);
        assert_eq!(number_sum("{}"), 0);
        assert_eq!(number_sum(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(number_sum(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(number_sum(r#"[1,"red",5]"#), 6);
    }
}
