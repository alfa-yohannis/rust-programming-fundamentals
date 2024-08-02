mod job_history {
    pub(crate) enum Employment {
        FullTime,
        PartTime,
        Freelance,
    }
}

pub fn calculate_salary() {
    let job1 = job_history::Employment::FullTime;
    let job2 = job_history::Employment::PartTime;
}
