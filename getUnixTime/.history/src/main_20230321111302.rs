use std::env;
use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc, DateTime};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let len = args.len() ;
    if len != 5 && len != 2 {
        println!("Please provide date and time in the format: (dd mm yyyy hh mm) or (timestamp) ");
        return;
    } 

    let (_, rest) = args.split_first_mut().unwrap();

if len == 6 {
    let p_args: Result<Vec<u32>, _> = rest.iter().map(|s| s.parse()).collect();
    let p_args = match p_args {
        Ok(p_args) => p_args,
        Err(err) => {
            println!("Error: invalid string format {:?}", err);
            return;
        }
    };

    let date_time: NaiveDateTime = NaiveDate::from_ymd_opt(p_args[2] as i32, p_args[1], p_args[0]).and_hms(p_args[3], p_args[4], 0);


    println!("{:?}", date_time.timestamp());}
    else {
  
        let timestamp: i64 = rest[0].parse::<i64>().unwrap() * 1000;
        let naive = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
        let date_time = DateTime::<Utc>::from_utc(naive, Utc);
        let rfc = date_time.to_rfc2822();
        println!("{:?}", rfc);
    }
}