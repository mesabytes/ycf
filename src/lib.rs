pub mod neoconf {
    pub struct Neoconf {
        file_path: String,
    }

    impl Neoconf {
        pub fn load(file_path: String) -> Self {
            Self { file_path }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::neoconf;

    #[test]
    fn it_works() {
        let config = neoconf::Neoconf::load(String::from("test.neoconf"));
        
        assert_eq!(2 + 2, 4);
    }
}
