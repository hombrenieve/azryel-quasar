use chrono::{DateTime, Local, Weekday, Duration, NaiveTime};
use std::collections::HashSet;

pub struct Habit {
    pub id: String,
    pub summary: String,
    pub streak: i32,
    pub stats: (i32, i32),
    frequency: Frequency
}

pub struct Frequency {
    days_of_week: std::collections::HashSet<Weekday>,
    schedule: Schedule,
    start_date: DateTime<Local>
}

pub struct Schedule {
    duration: Duration,
    start_times: Vec<NaiveTime>
}

impl Habit {
    pub fn new(id: &str, summary: &str, freq: Frequency) -> Habit {
        Habit {
            id: String::from(id), 
            summary: String::from(summary), 
            streak: 0,
            stats: (0, 0),
            frequency: freq
        }
    }
    pub fn next_sched(self) -> DateTime<Local> {
        self.frequency.next()
    }
}

impl Frequency {
    pub fn new(days: HashSet<Weekday>, starting: DateTime<Local>, schedule: Schedule) -> Frequency {
        Frequency {
            days_of_week: days,
            schedule: schedule,
            start_date: starting
        }
    }

    fn next(self) -> DateTime<Local> {
        self.schedule.next()
    }
}

impl Schedule {
    pub fn new(duration: Duration, start_times: Vec<NaiveTime>) -> Schedule {
        Schedule {
            duration: duration,
            start_times: start_times
        }
    }

    fn next(self) -> DateTime<Local> {
        Local::now().with_time(self.start_times[0]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct HabitBuilder {
        def_habit: Habit
    }

    impl HabitBuilder {
        fn new() -> HabitBuilder {
            let sched = Schedule::new(Duration::zero(), vec![]);
            let freq = Frequency::new(HashSet::new(), Local::now(), sched);
            HabitBuilder {
                def_habit: Habit::new("nohabit", "Nothing really", freq)
            }
        }

        fn id(mut self, new_id: &str) -> HabitBuilder {
            self.def_habit.id = String::from(new_id);
            self
        }

        fn summary(mut self, new_summary: &str) -> HabitBuilder {
            self.def_habit.summary = String::from(new_summary);
            self
        }

        fn sched_once(mut self, when: NaiveTime) -> HabitBuilder {
            self.def_habit.frequency.schedule.start_times = vec![when];
            self
        }

        fn freq_daily(mut self) -> HabitBuilder {
            let v = vec![Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri, Weekday::Sat, Weekday::Sun];
            self.def_habit.frequency.days_of_week = HashSet::from_iter(v.iter().cloned());
            self
        }

        fn build(self) -> Habit {
            self.def_habit
        }
    }

    #[test]
    fn test_new_habit() {
        let h = HabitBuilder::new()
                            .id("one")
                            .summary("long habit summary")
                            .build();
        assert_eq!(h.id, "one");
        assert_eq!(h.summary, "long habit summary");
    }

    #[test]
    fn test_schedule_next_soon() {
        let midnight = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let h = HabitBuilder::new()
                            .id("two")
                            .summary("midnight habit")
                            .sched_once(midnight)
                            .freq_daily()
                            .build();
        let when = Local::now().with_time(midnight).unwrap();
        assert_eq!((h.next_sched()-when).num_minutes(), 0);        
    }
}