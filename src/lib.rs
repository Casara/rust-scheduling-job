use chrono::{DateTime, Duration, Local};
use itertools::Itertools;
use std::ops::Sub;

const MAX_TIME: u32 = 8;

#[derive(Clone)]
pub struct Job {
    id: u64,
    description: String,
    max_end_date: DateTime<Local>,
    estimated_time: u32,
}

impl Job {
    pub fn get_start_date(&self) -> DateTime<Local> {
        let duration = Duration::hours(self.estimated_time as i64);
        self.max_end_date.sub(duration)
    }
}

pub struct ExecutionWindow {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

impl ExecutionWindow {
    pub fn is_within(&self, job: &Job) -> bool {
        self.check_date(&job.get_start_date()) && self.check_date(&job.max_end_date)
    }

    fn check_date(&self, date: &DateTime<Local>) -> bool {
        date.ge(&self.start) && date.le(&self.end)
    }
}

pub struct Schedule;

impl Schedule {
    pub fn order(window: &ExecutionWindow, jobs: &[Job]) -> Vec<Vec<Job>> {
        let mut count: u32 = 0;
        let mut index: u32 = 0;

        let mut arrays = Vec::new();

        let grouped = jobs
            .iter()
            .filter(|job| window.is_within(job))
            .sorted_by(|a, b| Ord::cmp(&a.max_end_date, &b.max_end_date))
            .group_by(move |job| {
                count += job.estimated_time;
                if count > MAX_TIME {
                    count = 0;
                    index += 1;
                }
                index
            });

        for (_, group) in &grouped {
            arrays.push(group.cloned().collect());
        }

        arrays
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{offset::TimeZone, Local};
    use pretty_assertions::assert_eq;

    struct Setup {
        jobs: Vec<Job>,
        window: ExecutionWindow,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                jobs: vec![
                    Job {
                        id: 1,
                        description: "Importação de arquivos de fundos".to_string(),
                        max_end_date: Local.ymd(2019, 11, 10).and_hms(12, 0, 0),
                        estimated_time: 2,
                    },
                    Job {
                        id: 2,
                        description: "Importação de dados da Base Legada".to_string(),
                        max_end_date: Local.ymd(2019, 11, 11).and_hms(12, 0, 0),
                        estimated_time: 4,
                    },
                    Job {
                        id: 3,
                        description: "Importação de dados de integração".to_string(),
                        max_end_date: Local.ymd(2019, 11, 11).and_hms(8, 0, 0),
                        estimated_time: 6,
                    },
                    // Jobs outside the execution window.
                    Job {
                        id: 4,
                        description: "Importação de arquivos de ...".to_string(),
                        max_end_date: Local.ymd(2019, 11, 10).and_hms(12, 0, 0),
                        estimated_time: 6,
                    },
                    Job {
                        id: 5,
                        description: "Importação de dados da ...".to_string(),
                        max_end_date: Local.ymd(2019, 11, 10).and_hms(5, 0, 0),
                        estimated_time: 5,
                    },
                    Job {
                        id: 6,
                        description: "Importação de dados de ...".to_string(),
                        max_end_date: Local.ymd(2019, 11, 11).and_hms(18, 0, 0),
                        estimated_time: 4,
                    },
                ],
                window: ExecutionWindow {
                    start: Local.ymd(2019, 11, 10).and_hms(9, 0, 0),
                    end: Local.ymd(2019, 11, 11).and_hms(12, 0, 0),
                },
            }
        }
    }

    #[test]
    fn test_job_get_start_date() {
        let expected_dates = vec![
            Local.ymd(2019, 11, 10).and_hms(10, 0, 0),
            Local.ymd(2019, 11, 11).and_hms(8, 0, 0),
            Local.ymd(2019, 11, 11).and_hms(2, 0, 0),
            Local.ymd(2019, 11, 10).and_hms(6, 0, 0),
            Local.ymd(2019, 11, 10).and_hms(0, 0, 0),
            Local.ymd(2019, 11, 11).and_hms(14, 0, 0),
        ];
        let setup = Setup::new();

        (0..setup.jobs.len())
            .for_each(move |i| assert_eq!(expected_dates[i], setup.jobs[i].get_start_date()));
    }

    #[test]
    fn test_is_within_the_execution_window() {
        let setup = Setup::new();

        setup
            .jobs
            .iter()
            .for_each(|job| assert_eq!(job.id <= 3, setup.window.is_within(job)));
    }

    #[test]
    fn test_schedule_order() {
        let setup = Setup::new();

        let arrays = Schedule::order(&setup.window, &setup.jobs);

        let expected: Vec<Vec<u64>> = vec![vec![1, 3], vec![2]];

        let output = arrays
            .iter()
            .map(|arr| arr.iter().map(|job| job.id).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>();

        assert_eq!(expected, output);
    }
}
