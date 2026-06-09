use crate::shell::{CommandError, RunningJob};

pub fn handle_jobs(jobs: &Vec<RunningJob>) -> Result<String, CommandError> {
    for (i, job) in jobs.iter().enumerate() {
        let is_most_recent = i == jobs.len() - 1;
        let most_recent_symbol = match is_most_recent {
            true => "+",
            false => "",
        };
        let pad_length = 24 - job.status.len();
        let mut padded_status = job.status.clone();
        let number = job.job_number.clone();
        let command_name = job.command_string.clone();
        padded_status.push_str(&" ".repeat(pad_length));
        let string_thing = format!(
            "[{}]{}  {}{}",
            number, most_recent_symbol, padded_status, command_name
        );
        println!("{}", string_thing);
    }
    Ok(String::new())
}
