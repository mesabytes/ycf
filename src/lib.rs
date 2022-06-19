pub mod neoconf {
    pub struct Neoconf {
        file_path: String,
    }

    impl Neoconf {
        pub fn new(file_path: String) -> Self {
            Self { file_path }
        }

        pub fn get(&self) {
            todo!();
        }

        pub fn set(&self, section: &str, key: &str, value: &str) {
            todo!();
        }

        pub fn remove(&self, section: &str, key: &str) {
            todo!();
        }

        /// read config file and return file content
        fn load_file(&self) -> String {
            let mut content = String::new();

            return content;
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::neoconf;

    #[test]
    fn it_works() {
        let config = neoconf::Neoconf::new(String::from("test.neoconf"));
        
        assert_eq!(2 + 2, 4);
    }
}
