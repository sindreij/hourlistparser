use std::{
    collections::HashMap,
    io::Read,
    ops::{Add, AddAssign, Sub},
};

use lazy_regex::regex_captures;

#[derive(Default, Clone, Debug)]
struct Line {
    start_time: Time,
    project: String,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.lines();

    let mut projects = HashMap::<String, Time>::new();
    let mut last_line: Option<Line> = None;

    for line in lines {
        if let Some((_, _, h, m, rest)) = regex_captures!(r#"(- )?(\d\d):(\d\d) (.*)"#, line) {
            let start_time = Time {
                hour: h.parse().unwrap(),
                minute: m.parse().unwrap(),
            };

            if let Some(last_line) = last_line.as_ref() {
                *projects.entry(last_line.project.to_owned()).or_default() +=
                    start_time - last_line.start_time;
            }

            last_line = Some(Line {
                start_time,
                project: rest.to_owned(),
            });
        }
    }

    if let Some(last_line) = last_line.as_ref() {
        if last_line.project != "Ferdig" {
            println!("Siste linje må være `Ferdig`");
            return;
        }
    }

    for (key, time) in projects {
        println!(
            "{}: {}",
            key,
            (time.hour as f64) + (time.minute as f64) / 60.
        );
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Time {
    hour: isize,
    minute: isize,
}

impl Add for &Time {
    type Output = Time;

    fn add(self, rhs: &Time) -> Self::Output {
        let mut hour = self.hour + rhs.hour;
        let mut minute = self.minute + rhs.minute;

        if minute >= 60 {
            hour += 1;
            minute -= 60;
        }

        Time { hour, minute }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Time) -> Self::Output {
        let mut hour = self.hour - rhs.hour;
        let mut minute = self.minute - rhs.minute;

        if minute < 0 {
            hour -= 1;
            minute += 60;
        }

        Time { hour, minute }
    }
}

impl AddAssign<Time> for Time {
    fn add_assign(&mut self, rhs: Self) {
        *self = &*self + &rhs;
    }
}
