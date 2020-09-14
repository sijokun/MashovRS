use json::JsonValue;

pub struct Session {
    csrf_token: String,
    session_id: String,
    user_id: String
}

impl Session {
    pub async fn get_timetable(&mut self) -> Result<TimeTable, String> {
        let client = reqwest::Client::new();
        let url: String = format!("https://web.mashov.info/api/students/{}/timetable", &self.user_id);

        let res = match client.get(&url)
            .header("User-Agent", "SLNK MASHOV BOT V0.1 by @luck20yan")
            .header("X-Csrf-Token", &self.csrf_token)
            .header("Cookie", format!("MashovSessionID={}; Csrf-Token={}", &self.session_id, &self.csrf_token))
            .send()
            .await {
            Ok(v) => v,
            Err(e) => { return Err(e.to_string()) }
        };

        let lessons = TimeTable {
            json: match json::parse(match &res.text().await {
                Ok(v) => v,
                Err(e) => { return Err(e.to_string()) }
            }) {
                Ok(v) => v,
                Err(e) => { return Err(e.to_string()) }
            }
        };

        Ok(lessons)
    }
    pub async fn get_homework(&mut self) -> Result<HomeWork, String> {
        let client = reqwest::Client::new();
        let url: String = format!("https://web.mashov.info/api/students/{}/homework", &self.user_id);

        let res = match client.get(&url)
            .header("User-Agent", "SLNK MASHOV BOT V0.1 by @luck20yan")
            .header("Accept", "application/json, text/plain, */*")
            .header("X-Csrf-Token", &self.csrf_token)
            .header("Connection", "keep-alive")
            .header("Cookie", format!("MashovSessionID={}; Csrf-Token={}", &self.session_id, &self.csrf_token))
            .send()
            .await {
            Ok(v) => v,
            Err(e) => { return Err(e.to_string()) }
        };

        let homework = HomeWork {
            homework: match json::parse(match &res.text().await {
                Ok(v) => v,
                Err(e) => { return Err(e.to_string()) }
            }) {
                Ok(v) => v,
                Err(e) => { return Err(e.to_string()) }
            }
        };

        Ok(homework)
    }
    pub async fn login(username: String, password: String) -> Result<Session, String> {
        let client = reqwest::Client::new();
        let res = client.post("https://web.mashov.info/api/login")
            .body(format!("{{\"semel\":480178,\"year\":2021 ,\"username\":\"{}\",\"password\":\"{}\",\"deviceModel\":\"SLNK MASHOV BOT\",\"deviceVersion\":\"0.1\"}}", username, password))
            .header("User-Agent", "SLNK MASHOV BOT V0.1 by @luck20yan")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Content-Type", "application/json")
            .send()
            .await.unwrap();


        let headers = res.headers().clone();
        let text = res.text().await.unwrap();

        if text.contains("error") {
            return Err(json::parse(&text).unwrap()["message"].to_string());
        } else {
            let mut csrf_token: String = String::new();
            let mut session_id: String = String::new();
            for i in headers {
                if match &i.0 { // crutch
                    None => "set-cookie".to_string(),
                    Some(d) => d.to_string()
                } == "set-cookie" {
                    let cookies: Vec<&str> = i.1.to_str().unwrap().split("; ").collect();
                    let cookie: &str = cookies[0];

                    if cookie.contains("Csrf-Token") {
                        let text: Vec<&str> = cookie.split("=").collect();
                        csrf_token = text[1].to_string();
                    } else if cookie.contains("MashovSessionID") {
                        let text: Vec<&str> = cookie.split("=").collect();
                        session_id = text[1].to_string();
                    }
                }
            }

            let login_json: JsonValue = json::parse(&text).unwrap();
            let session = Session {
                csrf_token: csrf_token,
                session_id: session_id,
                user_id: login_json["credential"]["userId"].to_string()
            };

            Ok(session)
        }
    }
}

pub struct HomeWork {
    pub homework: JsonValue
}

// impl HomeWork {
    // pub async fn send_to_telegam(&self, chat_id: String) {
    //     let client = reqwest::Client::new();
    //     for i in 0..(&self.homework).len() {
    //         let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}: {}", "BOT TOKEN", chat_id, &self.homework[i]["subjectName"], &self.homework[i]["homework"]);
    //         let res = client.get(&url)
    //             .send()
    //             .await;
    //         println!("{}: {}", &self.homework[i]["subjectName"], &self.homework[i]["homework"]);
    //     }
    // }
// }

pub struct TimeTable {
    pub json: JsonValue
}
impl TimeTable {
    // pub async fn send_to_telegam(&self, chat_id: String, days: Vec<usize>) {
    //     let mut text = String::from("TimeTable:\n\n");
    //     for day in days {
    //         text += format!("day: {}\n", day).as_ref();
    //         for j in 0..self.json[day-1].len() {
    //             text += format!("{} .{}   \n", &self.json[day-1][j]["groupDetails"]["groupName"], &self.json[day-1][j]["timeTable"]["lesson"]).as_ref();
    //         }
    //     }
    //     println!("{}", text);
    //     // here code for sending text to telegram
    // }

    pub async fn sort(&mut self) -> TimeTable{
        let a: JsonValue = self.json.clone();
        let mut lessons: Vec<Vec<JsonValue>> = vec![vec![],
                                                    vec![],
                                                    vec![],
                                                    vec![],
                                                    vec![],
                                                    vec![],
                                                    vec![],
        ];

        for i in 0..a.len() {
            lessons[(a[i]["timeTable"]["day"].as_i8().unwrap() - 1) as usize].push(a[i].clone());
        };
        for i in 0..lessons.len() {
            let mut m = lessons[i].len();
            while m > 0 {
                for j in 0..m-1 {
                    if &lessons[i][j]["timeTable"]["lesson"].as_i8().unwrap() > &lessons[i][j+1]["timeTable"]["lesson"].as_i8().unwrap() {
                        let tmp: JsonValue = lessons[i][j].clone();
                        lessons[i][j] = lessons[i][j+1].clone();
                        lessons[i][j+1] = tmp;
                    }
                }
                m=m-1;
            }

        };
        return TimeTable {
            json: JsonValue::from(lessons)
        };
    }
}