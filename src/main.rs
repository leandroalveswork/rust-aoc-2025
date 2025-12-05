mod mass_parser;
mod day05;

#[tokio::main]
async fn main() {
    day05::answer().await;
    day05::answer2().await;
}
