use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: usize,
    pub company: String,
    pub role: String,
    pub status: String,
    pub applied_at: DateTime<Local>,
}

fn data_file() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    path.push("jobs.json");
    Ok(path)
}

pub fn load_jobs() -> Result<Vec<Job>> {
    let path = data_file()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(path)?;
    let jobs = serde_json::from_str(&data)?;
    Ok(jobs)
}

pub fn save_jobs(jobs: &[Job]) -> Result<()> {
    let path = data_file()?;
    let json = serde_json::to_string_pretty(jobs)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn next_id(jobs: &[Job]) -> usize {
    jobs.iter().map(|j| j.id).max().unwrap_or(0) + 1
}
