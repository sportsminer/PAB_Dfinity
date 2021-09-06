use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, CandidType)]
pub struct Group {
    pub group_members: Vec<String>
}
