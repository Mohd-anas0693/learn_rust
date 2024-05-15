use candid::{CandidType, Deserialize};

#[derive(CandidType,PartialEq,Debug, Deserialize, Clone)]
pub struct PostData {
    pub title: String,
    pub data: String,
    pub created_by: String,
}
