use std::sync::Arc;

use gemini_rs::prelude::{GeminiClient, TokenProvider};
use serde::{Deserialize, Serialize};

const SYSTEM_PROMPT: &str = "You are an expert proofreader. Analyze the provided text and identify any corrections needed to fix grammar or spelling. For each mistake, identify the startIndex, endIndex, the mistake type, report the correction and an explanation. Finally, provide the entire corrected string.";

#[derive(Serialize, Deserialize)]
pub struct Proofreading {
    pub corrected: String,
    pub corrections: Vec<Correction>,
}

#[derive(Serialize, Deserialize)]
pub struct  Correction {
    pub start_index: usize,
    pub end_index: usize,
    pub correction: String,
    #[serde(rename = "type")]
    pub correction_type: CorrectionType,
    pub explanation: String,
}

#[derive(Serialize, Deserialize)]
pub enum CorrectionType {
    #[serde(rename = "spelling")]
    Spelling,
    #[serde(rename = "punctuation")]
    Punctuation,
    #[serde(rename = "capitalization")]
    Capitalization,
    #[serde(rename = "preposition")]    
    Preposition,
    #[serde(rename = "missing-words")]        
    MissingWords,
    #[serde(rename = "grammar")]        
    Grammar,
}

pub async fn proofread<T: TokenProvider + Clone>(vertex_client: GeminiClient<T>, input: &str) -> gemini_rs::error::Result<Proofreading> {
    todo!()
}
