use urlencoding::encode;

pub fn build_query_string(params: &Vec<(&str, String)>) -> String {
    let mut query_string = String::new();
    for (i, (key, value)) in params.iter().enumerate() {
        query_string.push_str(&encode(key));
        query_string.push_str("=");
        query_string.push_str(&encode(&value));
        if i < params.len() - 1 {
            query_string.push_str("&");
        }
    }
    query_string
}
