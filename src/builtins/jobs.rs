use crate::shell::{CommandError, RunningJob};

pub fn handle_jobs(jobs: &mut Vec<RunningJob>) -> Result<String, CommandError> {
    let len = jobs.len();
    for (i, job) in jobs.iter_mut().enumerate() {
        let mut most_recent_symbol = "";
        if len > 0 && i == len - 1 {
            most_recent_symbol = "+"
        }
        if len > 1 && i == len - 2 {
            most_recent_symbol = "-"
        }
        if matches!(job.process_info.try_wait(), Ok(Some(_))) {
            job.status = "Done".to_string();
        }
        let pad_length = 24 - job.status.len();
        let mut padded_status = job.status.clone();
        let number = job.job_number.clone();
        let mut command_name = job.command_string.clone();
        if job.status == "Done".to_string() {
            command_name = command_name.strip_suffix(" &").unwrap().to_string();
        }
        padded_status.push_str(&" ".repeat(pad_length));
        let string_thing = format!(
            "[{}]{}  {}{}",
            number, most_recent_symbol, padded_status, command_name
        );
        println!("{}", string_thing);
    }
    jobs.retain_mut(|job| job.status == "Running".to_string());
    Ok(String::new())
}
