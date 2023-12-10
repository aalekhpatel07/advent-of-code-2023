use reqwest::blocking::Client;

pub fn submit(day: u32, part: u32, solution: &str) -> Result<bool, String> {
    println!("Day {day:02} part {part}: {solution}");
    let url = format!("https://adventofcode.com/2023/day/{day}");
    let client = Client::new();

    let form: std::collections::HashMap<String, String> = [
        (String::from("answer"), solution.to_string()),
        (String::from("level"), part.to_string()),
    ]
    .into_iter()
    .collect();

    let session = std::env::var("AOC_SESSION").expect("AOC_SESSION to be present in the env.");

    let res = client
        .post(url)
        .form(&form)
        .header("Cookie", format!("session={session}"))
        .send()
        .expect("couldn't make request");

    if res.status() != 200 {
        eprintln!(
            "{}\n\n{}",
            res.status(),
            String::from_utf8(res.bytes().unwrap().to_vec()).unwrap()
        );
        return Err("received a bad response from the server.".to_string());
    }
    let contents =
        String::from_utf8(res.bytes().unwrap().to_vec()).expect("received invalid utf-8 response");
    let needle = "That's not the right answer";

    if contents.contains(needle) {
        return Ok(false);
    }
    Ok(true)
}
