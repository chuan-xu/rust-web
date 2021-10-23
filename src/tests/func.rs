#[cfg(test)]
mod tests {
    use crate::mode::{self, ExtJson};

    #[test]
    fn dese() {
        let mut v: Vec<mode::user::User> = Vec::new();
        let str = r#"{"id":1, "uid":"123", "uname":"chuan1"}.{"id":1, "uid":"2", "uname":"chuan2"}.{"id":1, "uid":"123", "uname":"chuan3"}"#;
        v = v.ext_deserialize(str).unwrap();
        for i in v.iter() {
            println!("{:?}", i);
        }
    }

    #[test]
    fn sstr() {
        let s = "hh".to_string();
        let v: Vec<&str> = s.split(".").collect();
        println!("{:?}", v);
    }
}