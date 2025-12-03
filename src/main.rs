mod mass_parser;
mod day01;
mod day02;
mod day03;

#[tokio::main]
async fn main() {
    day03::answer().await;
    day03::answer2().await;
}
