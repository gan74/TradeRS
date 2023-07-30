use curl::easy::{Easy2, Handler, WriteError, List};

use std::io::Write;


#[derive(Debug)]
pub enum RequestError {
    GenericError,
}


pub struct RequestManager {
    log_file: Box<dyn Write>,
}

impl RequestManager {
    pub fn new<F: Write + 'static>(file: F) -> RequestManager {
        RequestManager {
            log_file: Box::new(file),
        }
    }

    pub fn post(&mut self, url: &str, token: &str) -> Result<String, RequestError> {
        write!(self.log_file, "-------------------------------- POST --------------------------------\nto: {url}\n\n").unwrap();
        let res = curl(url, token, RequestType::Post);
        self.log_result(&res);
        res
    }

    pub fn get(&mut self, url: &str, token: &str) -> Result<String, RequestError> {
        write!(self.log_file, "-------------------------------- GET --------------------------------\nto: {url}\n\n").unwrap();
        let res = curl(url, token, RequestType::Get);
        self.log_result(&res);
        res
    }


    fn log_result(&mut self, res: &Result<String, RequestError>) {
        match res {
            Ok(result) => write!(self.log_file, "result:\n{}\n\n\n\n", result).unwrap(),
            Err(error) => write!(self.log_file, "error:\n{:?}\n\n\n\n", error).unwrap(),
        }
    }
}




enum RequestType {
    Get,
    Post,
}

fn curl(url: &str, token: &str, req_type: RequestType) -> Result<String, RequestError> {
    let mut easy = Easy2::new(Collector(Vec::new()));

    let mut list = List::new();
    list.append(&format!("Authorization: Bearer {token}"))?;
    easy.http_headers(list)?;

    match req_type {
        RequestType::Get => easy.get(true)?,
        RequestType::Post => easy.post(true)?,
    }

    easy.url(url)?;
    easy.perform()?;
 
    let contents = easy.get_ref();
    Ok(String::from_utf8_lossy(&contents.0).to_string())
}

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}


impl From<curl::Error> for RequestError {
    fn from(_: curl::Error) -> Self {
        RequestError::GenericError
    }
}