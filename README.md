# Flight Cancellation Prediction and Carrier Recommendation

This repository contains a Rust program designed to predict flight cancellations or recommend the best carrier for an upcoming flight. It interacts with a database containing flight, airport, and carrier data to provide insights based on historical records.

## Usage

### Requirements
- Rust programming language
- `reqwest`, `serde_json`, and `tokio` crates

### Setup
1. Clone the repository to your local machine.
2. Ensure you have Rust and SpiceOSS Runtime installed.
3. ``` cd /spice_qs ```
4. ``` spice run ```
5. Use ``` cargo run ``` to run the program.
6. Follow the prompts to select an option:
    - Option 1: Predict Cancellation
    - Option 2: Find the best carrier for an upcoming flight

## Code Overview

The main functionality of the program is provided by the `main` function. It interacts with the user via standard input (`stdin`) and executes SQL queries to retrieve relevant flight, carrier, and airport information.

The program utilizes asynchronous Rust features (`tokio`) to handle I/O operations efficiently. It also makes use of the `reqwest` crate to send HTTP requests to a local server hosting the database.

## Database Interaction

The program interacts with a local database to retrieve historical flight data and carrier information. It executes SQL queries dynamically to fetch data relevant to the user's input.

### Datasets Used:
- [Flight Datasets](https://www.kaggle.com/datasets/yuanyuwendymu/airline-delay-and-cancellation-data-2009-2018/data?select=2016.csv)
- [Airport Dataset](https://www.kaggle.com/datasets/thoudamyoihenba/airports)
- [Carrier Dataset - Included in folder]([https://www.kaggle.com/datasets/thoudamyoihenba/airports](https://courses.cs.washington.edu/courses/cse414/19au/hw/flight-dataset.zip))

## Functionality

### Predict Cancellation (Option 1)
- Allows users to predict the likelihood of flight cancellation based on historical data.
- Users input the origin, destination, and carrier, and the program calculates the percentage of previous flights by that carrier on the same route that were cancelled.

### Find Best Carrier (Option 2)
- Recommends the best carrier for an upcoming flight based on historical cancellation data.
- Users input the origin and destination, and the program identifies the carrier with the least number of cancellations on that route.
