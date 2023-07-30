use serde_json::Value;

#[derive(Debug)]
pub struct Agent {
    id: String,
    symbol: String,
    pub headquarters: String,
    faction: String,

    credits: i64,
}

impl Agent {
    pub fn from_json(value: &serde_json::Value) -> Result<Agent, ()> {
        match (&value["accountId"], &value["symbol"], &value["headquarters"], &value["startingFaction"], &value["credits"]) {
            (Value::String(id), Value::String(sym), Value::String(hd), Value::String(faction), Value::Number(creds)) => {
                Ok(Agent{
                    id: id.clone(),
                    symbol: sym.clone(),
                    headquarters: hd.clone(),
                    faction: faction.clone(),
                    credits: creds.as_i64().expect("Credits are not i64"),
                })
            },

            _ => Err(()),
        }
    }
}