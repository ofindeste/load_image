#[derive(Debug)]
pub struct State {
    client: reqwest::Client,
}

impl State {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn load_image(&self, url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut res = self.client.get(url).send()?;

        if res.status().is_success() {
            let mut s = Vec::<u8>::new();
            res.copy_to(&mut s)?;

            Ok(s)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,
                         "Could not get resource successfully"))
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs::File, io::Write};

    #[test]
    fn check_state_load_image() {
        let state = State::new();

        let res = state.load_image("https://sr.gallerix.ru/_UNK/1018810316/3526.jpg").unwrap();
        let mut file = File::create("test_assets/res.jpg").unwrap();
        file.write(&res).unwrap();
        let expected_image = std::fs::File::open("test_assets/img.jpg").unwrap();

        let result_metadata = file.metadata().unwrap();
        let expected_metadata = expected_image.metadata().unwrap();

        assert_eq!(result_metadata.len(), expected_metadata.len());
    }
}
