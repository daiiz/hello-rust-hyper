use std::collections::HashMap;

lazy_static! {
    static ref ENDPOINTS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        map.insert("hello", "/hello");
        map.insert("num", r"/num/(?P<a>\d+)/(?P<b>\d*)");
        map.insert("root", "/");

        map
    };
}

pub(crate) fn p(name: &str) -> &'static str {
  ENDPOINTS.get(name).unwrap()
}
