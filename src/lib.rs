use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Job {
    id: u64,
    description: String,
    max_end_date: DateTime<Local>,
    estimated_time: u32,
}

pub struct ExecutionWindow {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

pub struct Schedule;

impl Schedule {
    pub fn order(window: &ExecutionWindow, jobs: &[Job]) -> Vec<Vec<Job>> {
        unimplemented!()
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
