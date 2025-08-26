use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    //Open a connection to mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    //Set the key "hello" to the value "world"
    client.set("hello", "world".into()).await?;

    //Get key hello
    let result = client.get("hello").await?;

    println!("got the value from the server; result:{:?}", result);

    Ok(())
}
