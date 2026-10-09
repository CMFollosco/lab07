use rand::prelude::*;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}

fn main() {
    let mut log: Vec<Reading> = vec![
        Reading {
            day: DAYS[0].to_string(),
            high: 72,
        },
        Reading {
            day: DAYS[1].to_string(),
            high: 68,
        },
        Reading {
            day: DAYS[2].to_string(),
            high: 75,
        },
        Reading {
            day: DAYS[3].to_string(),
            high: 81,
        },
        Reading {
            day: DAYS[4].to_string(),
            high: 79,
        },
    ];

    let mut rng = rand::rng();

    log.push(Reading {
        day: DAYS[5].to_string(),
        high: rng.random_range(60..=100),
    });

    log.push(Reading {
        day: DAYS[6].to_string(),
        high: rng.random_range(60..=100),
    });

    println!("Temperature Log:");

    for i in 0..log.len() {
        println!("{}: {}", log[i].day, log[i].high);
    }

    let summary = summarize(&log, 75);

    println!("Average temp: {:.2}", summary.average);
    println!("Hottest day: {}", summary.hottest_day);
    println!("Days above 75: {}", summary.days_above);
}

fn average_temp(log: &Vec<Reading>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }

    let mut sum = 0;

    for reading in log {
        sum += reading.high;
    }

    sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<Reading>) -> usize {
    let mut index = 0;

    for i in 0..log.len() {
        if log[i].high > log[index].high {
            index = i;
        }
    }

    index
}

fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    let mut count = 0;

    for temp in log {
        if temp.high > threshold {
            count += 1;
        }
    }

    count
}

fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
    Summary {
        average: average_temp(log),
        hottest_day: log[hottest_day(log)].day.clone(),
        days_above: count_above(log, threshold),
    }
}
