use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)] 
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}
