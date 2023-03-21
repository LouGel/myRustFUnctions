use std::env;
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let len = args.len() ;
    
    if len < 6 {
        println!("Please provide date and time in the format: dd mm yyyy hh mm");
        return;
    }

    let (_, rest) = args.split_first_mut().unwrap();


    // let p_args: Vec<u32> = args.iter().map(|s| s.parse().unwrap()).collect();
    let p_args: Result<Vec<u32>, _> = rest.iter().map(|s| s.parse()).collect();
    let p_args = match p_args {
        Ok(p_args) => p_args,
        Err(err) => {
            println!("Error: invalid string format {:?}", err);
            return;
        }
    };

    let date_time: NaiveDateTime = NaiveDate::from_ymd(p_args[2] as i32, p_args[1], p_args[0]).and_hms(p_args[3], p_args[4], 0);


    println!("{:?}", date_time.timestamp());
}