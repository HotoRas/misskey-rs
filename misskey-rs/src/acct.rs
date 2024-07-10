pub mod Acct {
    pub struct Acct {
        pub username: String,
        pub host: Option<String>,
    }

    pub fn parse(acct: &String) -> Acct {
        let mut _acct: String;

        if acct.starts_with('@') {
            unsafe {
                _acct = acct.get_unchecked(1..acct.len()).to_string();
            }
        } else {
            _acct = acct.to_string();
        }

        let split: Vec<&str> = acct.split('@').collect();

        Acct {
            username: split[0].to_string(),
            host: {
                if split.len() > 1 {
                    Some(split[1].to_string())
                } else {
                    None
                }
            },
        }
    }

    pub fn toString(acct: &Acct) -> String {
        if acct.host == None {
            return acct.username.to_string();
        }

        format!("{}@{}", acct.username, acct.host.clone().unwrap())
    }
}
