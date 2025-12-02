mod mass_parser;
mod day01;
mod day02;

#[tokio::main]
async fn main() {
    day02::answer().await;
    day02::answer2().await;
}
