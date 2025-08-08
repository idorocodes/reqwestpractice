use reqwest;
use serde::Deserialize;
use std::error::Error;
use tokio::time::sleep;
use std::time::Duration;
#[derive(Deserialize)] // Allow us to parse json and use the struct
struct ResponseStruct {
    delivery:String,
    setup:String,
}

#[tokio::main] //this line makes the main function async !
async  fn main () -> Result<(), Box<dyn Error >> {
    let url = "https://v2.jokeapi.dev/joke/Programming"; // takes a string slice as the url
    let req  = reqwest::get(url).await?; // sends a get request to the joke's api endpoint
    let data : ResponseStruct = req.json().await?; // parse the incoming response to fit into our predefined ResponseStruct struct
    println!("The joke is : {}", data.setup);  // prints the joke question (the setup is the key that contains the question in the api
    sleep(Duration::from_millis(2000)).await; // adds 2 seconds delay .... added this just for fun
    println!("The answer is : {}", data.delivery); //  prints the answer after 2 seconds
    Ok(())
}
