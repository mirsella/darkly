use regex::Regex;
use std::{collections::HashSet, env::args};
use url::Url;

fn scrap(url: Url, set: &mut HashSet<String>) {
    let res = ureq::get(url.as_str())
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    if url.path().ends_with("README") && set.insert(res.trim().to_string()) {
        dbg!(res);
        return;
    }
    // FIXME: should not re parse a regex in a hot loop, it's fine for this little tool, we are IO bound anyway
    let regex = Regex::new(r#"(?m)^<a href="(\w+/?)">"#).unwrap();
    for m in regex.captures_iter(&res) {
        let path = m.get(1).unwrap().as_str();
        let url = url.join(path).unwrap();
        if url.path().starts_with("..") {
            continue;
        }
        scrap(url, set);
    }
}

fn main() {
    let mut url = Url::parse("http://localhost/.hidden/").unwrap();
    url.set_port(args().nth(1).map(|p| p.parse().expect("valid port")))
        .expect("can't set port on url");
    let mut set = HashSet::new();
    scrap(url, &mut set);
    dbg!(set);
}
