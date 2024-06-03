use chrono::{DateTime, Local, Weekday, Duration};

pub struct Habit {
    pub id: String,
    pub summary: String,
    pub streak: i32,
    pub stats: (i32, i32),
    pub frequency: Frequency
}

struct Frequency {
    days_of_week: std::collections::HashSet<Weekday>,
    schedule: Schedule,
    start_date: DateTime<Local>
}

struct Schedule {
    duration: Duration,
    start_times: Vec<DateTime<Local>>
}

impl Habit {
    pub fn new(id: &str, summary: &str) -> Habit {
        Habit {
            id: String::from(id), 
            summary: String::from(summary), 
            streak: 0,
            stats: (0, 0),
            frequency: Frequency::new()
        }
    }
}

impl Frequency {
    fn new() -> Frequency {
        Frequency {
            days_of_week: std::collections::HashSet::new(),
            schedule: Schedule::new(),
            start_date: Local::now()
        }
    }
}

impl Schedule {
    fn new() -> Schedule {
        Schedule {
            duration: Duration::new(1, 1).unwrap(),
            start_times: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_habit() {
        let h = Habit::new("one", "long habit summary");
        assert_eq!(h.id, "one");
        assert_eq!(h.summary, "long habit summary");
    }
}