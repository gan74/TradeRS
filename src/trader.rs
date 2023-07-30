use serde_json::Value;

pub struct Agent {
    token: String,
}

impl Agent {
    pub fn from_json(token_data: &str) -> Result<Agent, ()> {
        if let Ok(parsed) = serde_json::from_str::<Value>(token_data) {
            let token = parse_token(&parsed)?;

            Ok(Agent{
                token: token
            })
        } else {
            Err(())
        }
    }
}


fn parse_token(value: &Value) -> Result<String, ()> {
    match &value["data"]["token"] {
        Value::String(tk) => Ok(tk.clone()),
        _ => Err(())
    }
}