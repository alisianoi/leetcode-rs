use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut logs: Vec<LogEntry> = key_name
            .iter()
            .zip(key_time.iter())
            .map(|(name, time)| LogEntry {
                name: name.clone(),
                time: TimeEntry::new(time),
            })
            .collect();

        logs.sort();

        let mut lft = 0;
        let mut rgt = 0;
        let mut names: Vec<String> = vec![];
        while lft != logs.len() {
            while rgt != logs.len()
                && logs[lft].name == logs[rgt].name
                && logs[rgt].time.hh * 60 + logs[rgt].time.mm
                    <= logs[lft].time.hh * 60 + logs[lft].time.mm + 60
            {
                rgt += 1;
            }

            if rgt - lft >= 3 {
                names.push(logs[lft].name.clone());

                while rgt != logs.len() && logs[lft].name == logs[rgt].name {
                    rgt += 1;
                }
            }

            if rgt == logs.len() {
                break;
            }

            if logs[lft].name != logs[rgt].name {
                lft = rgt;
            } else {
                lft += 1;
            }
        }

        names
    }
}

#[derive(PartialOrd, PartialEq, Eq)]
struct LogEntry {
    name: String,
    time: TimeEntry,
}

impl Ord for LogEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.name != other.name {
            return self.name.cmp(&other.name);
        }

        self.time.cmp(&other.time)
    }
}

#[derive(PartialOrd, PartialEq, Eq)]
struct TimeEntry {
    hh: i32,
    mm: i32,
}

impl TimeEntry {
    pub fn new(time_entry: &str) -> Self {
        let xs: Vec<i32> = time_entry
            .split(':')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        TimeEntry {
            hh: xs[0],
            mm: xs[1],
        }
    }
}

impl Ord for TimeEntry {
    fn cmp(&self, other: &TimeEntry) -> Ordering {
        if self.hh != other.hh {
            return self.hh.cmp(&other.hh);
        }

        self.mm.cmp(&other.mm)
    }
}

fn main() {
    println!("Hello, world!");
}
