use reqwest::Client;
use std::collections::HashMap;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut reader = BufReader::new(tokio::io::stdin());
    let mut option = String::new();
    let mut carrier = String::new();
    let mut origin = String::new();
    let mut dest = String::new();

    println!("Would you like to: \n\nPredict Cancellation (1)\nFind best carrier for an upcoming flight (2)\n\n");
    reader.read_line(&mut option).await.unwrap();
    let option: u32 = option.trim().parse().expect("Please give a valid option (1-2)!");

    //origin and destination Airport Codes
    println!("\nWhere will you be flying from?");
    reader.read_line(&mut origin).await.unwrap();
    let origin = origin.replace("\n", "");
    let query = format!("SELECT \"IATA\" FROM airports as A WHERE A.\"IATA\" = '{origin}' OR A.\"City\" = '{origin}' OR A.\"Name\" = '{origin}' limit 1");
    let origin = execute_query(&query).await.unwrap()[0].clone();
    let origin = &origin["IATA"].to_string();
    let origin = origin.trim_matches('\"');

    println!("\nWhere are you going to?");
    reader.read_line(&mut dest).await.unwrap();
    let dest = dest.replace("\n", "");
    let query = format!("SELECT \"IATA\" FROM airports as A WHERE A.\"IATA\" = '{dest}' OR A.\"City\" = '{dest}' OR A.\"Name\" = '{dest}' limit 1");
    let dest = execute_query(&query).await.unwrap()[0].clone();
    let dest = &dest["IATA"].to_string();
    let dest = dest.trim_matches('\"');

    let mut query2 = String::new();
    if option % 2 == 1 {
        println!("\nWhat carrier will you be taking?");
        reader.read_line(&mut carrier).await.unwrap();
        let carrier = carrier.replace("\n", "");
        let query = format!("SELECT \"02Q\" FROM carriers WHERE \"02Q\" = '{carrier}' OR \"Titan Airways\" = '{carrier}' limit 1");
        let carrier = execute_query(&query).await.unwrap()[0].clone();
        let carrier = &carrier["02Q"].to_string();
        let carrier = carrier.trim_matches('\"');

        for year in 2009..=2018 {
            
            query2 += &format!(
                "SELECT 'flights_{year}_accelerated' AS table_name FROM flights_{year}_accelerated WHERE \"OP_CARRIER\" = '{carrier}' AND \"ORIGIN\" = '{origin}' AND \"DEST\" = '{dest}'"
            );

            if year < 2018 {
                query2 += "\nUNION\n";
            }
        }
    } else {

        for year in 2009..=2018 {
            if year != 2012 {
                query2 += &format!(
                    "SELECT 'flights_{year}_accelerated' AS table_name FROM flights_{year}_accelerated WHERE \"ORIGIN\" = '{origin}' AND \"DEST\" = '{dest}'"
                );

                if year < 2018 {
                    query2 += "\nUNION\n";
                }
            }
        }
    }
    let years = execute_query(&query2).await.unwrap();


    let carrier = carrier.replace("\n", "");
    match option % 2 == 1 {
        true => {
            let mut cancelled_count : f64 = 0.0;
            let mut total_count = 0;

            if years.as_array().expect("REASON").len() == 0{
                println!("There were no such flights on this airline");
            } else {
                for index in 0..years.as_array().expect("REASON").len() {
                    let query3 = format!("SELECT SUM(\"CANCELLED\") AS cancelled_flights_count, COUNT(*) AS total_count
                        FROM {} as flight_table
                        JOIN carriers as C ON C.\"02Q\" = flight_table.\"OP_CARRIER\"
                        WHERE \"OP_CARRIER\" = '{carrier}' AND \"ORIGIN\" = '{origin}' AND \"DEST\" = '{dest}'
                        GROUP BY \"OP_CARRIER\"", years[index]["table_name"]);
                    let cancel_json = execute_query(&query3).await.unwrap();
                    cancelled_count = cancelled_count as f64 + cancel_json[0]["cancelled_flights_count"].to_string().parse::<f64>().unwrap();
                    total_count = total_count + cancel_json[0]["total_count"].to_string().parse::<i64>().unwrap();
                }
    
                //calculates percentage
                let result = (cancelled_count / total_count as f64)*100.0;
                println!("If you fly with this carrier the percentage of cancellation is {}.", result);
            }
        },
        false => {
            let mut map: HashMap<String, i32> = HashMap::new();
            let mut most_appeared = String::new();

            for index in 0..years.as_array().expect("REASON").len() {
                let query3 = format!("SELECT C.\"Titan Airways\" as Carrier, SUM(\"CANCELLED\") AS cancelled_flights_count
                    FROM {} as flight_table
                    JOIN carriers as C ON C.\"02Q\" = flight_table.\"OP_CARRIER\"
                    WHERE \"ORIGIN\" = '{origin}' AND \"DEST\" = '{dest}'
                    GROUP BY \"OP_CARRIER\", C.\"Titan Airways\"
                    ORDER BY cancelled_flights_count LIMIT 1", years[index]["table_name"]);
                let cancel_json = execute_query(&query3).await.unwrap();

                //finds most appeared carrier
                map.insert(cancel_json[0]["carrier"].to_string(), map.get(&cancel_json[0]["carrier"].to_string()).unwrap_or(&0) + 1);
                if most_appeared != cancel_json[0]["carrier"].to_string() && 
                        map.get(&most_appeared).unwrap_or(&0) < map.get(&cancel_json[0]["carrier"].to_string()).unwrap() {
                    most_appeared = cancel_json[0]["carrier"].to_string();
                }
            }

            println!("The best carrier to fly with is {}", most_appeared);
        }
    }
    

    Ok(())
}

async fn execute_query(query: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:3000/v1/sql";
    let client = Client::new();

    let response = client.post(url)
        .body(query.to_owned())
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json_value: Value = serde_json::from_str(&body)?;
        Ok(json_value)
    } else {
        println!("Request failed with status: {}", response.status());
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}
