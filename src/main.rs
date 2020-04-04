use chrono::{DateTime, Local, TimeZone};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
struct MyTime {
    pub date_time: NaiveDateTime,
}

fn main() {
    let dt: DateTime<Local> = Local::now();
    let local = dt.naive_local();
    //2020-04-04 11:39:56.276808167
    println!("{}", local);
    let time = MyTime {
        date_time: local,
    };
    println!("{}", serde_json::to_string(&time).unwrap());
    let my: MyTime = serde_json::from_str(r#"{"date_time":"2020-04-04T23:50:02"}"#).unwrap();
    println!("{:?}", my);
}
