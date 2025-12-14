use clap::{Parser, Subcommand};
use anyhow::Result;
use chrono::Local;

use jobtrackr::{Job, load_jobs, save_jobs, next_id};

#[derive(Parser)]
#[command(name = "JobTrackr")]
#[command(about = "Track job applications from the command line")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        company: String,
        role: String,
        status: String,
    },
    List,
    Update {
        id: usize,
        status: String,
    },
    Delete {
        id: usize,
    },
    /// Show reminders for jobs older than X days
    Remind {
        #[arg(long)]
        days: Option<u32>, // Now works as --days
    },
    /// Filter jobs by status, company, or role
    Filter {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        company: Option<String>,
        #[arg(long)]
        role: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut jobs = load_jobs()?;

    match cli.command {
        Commands::Add { company, role, status } => {
            jobs.push(Job {
                id: next_id(&jobs),
                company,
                role,
                status,
                applied_at: Local::now(),
            });
            save_jobs(&jobs)?;
            println!("✅ Job added");
        }

        Commands::List => {
            if jobs.is_empty() {
                println!("No job applications found.");
            } else {
                for job in jobs {
                    println!(
                        "[{}] {} - {} | {} | {}",
                        job.id,
                        job.company,
                        job.role,
                        job.status,
                        job.applied_at.format("%Y-%m-%d")
                    );
                }
            }
        }

        Commands::Update { id, status } => {
            if let Some(job) = jobs.iter_mut().find(|j| j.id == id) {
                job.status = status;
                save_jobs(&jobs)?;
                println!("🔄 Job updated");
            } else {
                println!("❌ Job not found");
            }
        }

        Commands::Delete { id } => {
            let before = jobs.len();
            jobs.retain(|j| j.id != id);
            if before == jobs.len() {
                println!("❌ Job not found");
            } else {
                save_jobs(&jobs)?;
                println!("🗑️ Job deleted");
            }
        }

        Commands::Remind { days } => {
            let threshold_days = days.unwrap_or(7);
            let now = chrono::Local::now();
            let mut found = false;

            for job in &jobs {
                let age = now.signed_duration_since(job.applied_at).num_days();
                if age >= threshold_days as i64 {
                    println!(
                        "⏰ Reminder: [{}] {} - {} | Status: {} | Applied {} days ago",
                        job.id, job.company, job.role, job.status, age
                    );
                    found = true;
                }
            }

            if !found {
                println!("No jobs need reminders.");
            }
        }

        Commands::Filter { status, company, role } => {
            let filtered: Vec<_> = jobs.iter().filter(|job| {
                (status.is_none() || job.status.to_lowercase() == status.as_ref().unwrap().to_lowercase()) &&
                (company.is_none() || job.company.to_lowercase().contains(&company.as_ref().unwrap().to_lowercase())) &&
                (role.is_none() || job.role.to_lowercase().contains(&role.as_ref().unwrap().to_lowercase()))
            }).collect();

            if filtered.is_empty() {
                println!("No jobs match your filter criteria.");
            } else {
                for job in filtered {
                    println!(
                        "[{}] {} - {} | {} | {}",
                        job.id,
                        job.company,
                        job.role,
                        job.status,
                        job.applied_at.format("%Y-%m-%d")
                    );
                }
            }
        }
    }

    Ok(())
}
