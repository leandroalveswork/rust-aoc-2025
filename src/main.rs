mod mass_parser;
mod day01;
mod day02;
mod day03;
mod day04;

#[tokio::main]
async fn main() {
    day04::answer().await;
    day04::answer2().await;
}
