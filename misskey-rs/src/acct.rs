pub mod Acct {
    pub struct Acct {
        pub username: String;
        pub host: Option<String>;
    }

    pub fn parse(acct: &String) -> Option<Acct> {
        let mut _acct: String;
        
        if (acct.starts_with('@')) {
            unsafe {
                _acct = acct.get_unchecked(1.._acct.len())
            }
        } else {
            _acct = acct;
        }

        let split = acct.split('@').collect();

        Acct {
            username: split[0],
            host: split.len() > 1 ? Some(split[1]) : None
        }
    }

    pub fn toString(acct: &Acct) -> String {
        if (acct.host == None) {
            acct.username
        }

        format!("{}@{}", acct.username, acct.host)
    }
}