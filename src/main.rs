async fn fetch_data() -> Result<String, reqwest::Error> {
  let response = reqwest::get("https://httpbin.org/ip").await?;
  let text = response.text().await?;
  Ok(text)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("y después del closure: {}", fetch_data().await?);

  Ok(())
}
