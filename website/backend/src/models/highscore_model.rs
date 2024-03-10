use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Highscore {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub total_time: u32,
    pub total_score: u32,
    pub level_scores: HashMap<u8, u32>,
    pub level_times: HashMap<u8, u32>,
}

impl Highscore {
    pub fn cal_total_score(&mut self) {
        self.total_score = self.level_scores.values().sum();
    }
}