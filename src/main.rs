use std::collections::HashMap;
use crate::lib::*;
use config::Config;

mod lib;


#[tokio::main]
async fn main()  -> Result<(), String>{
    let mut settings: Config = config::Config::default();
    settings
        .merge(config::File::with_name("Settings")).unwrap();
    let conf = settings.try_into::<HashMap<String, String>>().unwrap();
    let username = String::from(&conf["username"]);
    let password = String::from(&conf["password"]);
    let mut user: Session = lib::Session::login(username, password).await?;


    let timetable: TimeTable = user.get_timetable().await?.sort().await;
    let days: Vec<usize> = vec![1, 2, 3];
    let mut text: String = String::from("TimeTable:\n");

    for day in days {
        text += format!("day: {}\n", day).as_ref();
        for j in 0..timetable.json[day-1].len() {
            text += format!("{} .{}   \n", &timetable.json[day-1][j]["groupDetails"]["groupName"], &timetable.json[day-1][j]["timeTable"]["lesson"]).as_ref();
        }
    }
    println!("{}", text);


    println!("=================================");


    let homework: HomeWork = user.get_homework().await?;

    for i in 0..(&homework.homework).len() {
                println!("{}: {}\n", &homework.homework[i]["subjectName"], &homework.homework[i]["homework"]);
            }

    Ok(())
}




