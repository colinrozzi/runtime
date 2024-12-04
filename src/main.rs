// okay. this project will take an actor, and will run that actor and create a hash chain along the
// way.
use runtime::Actor;
use std::error::Error;

// what is the question right now?
// I don't know the proper w

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut actor1 = Actor::new(
        "../actor1/target/wasm32-unknown-unknown/release/actor1.wasm".to_string(),
        "actor1".to_string(),
    );
    actor1.start().await?;

    let mut actor2 = Actor::new(
        "../actor2/target/wasm32-unknown-unknown/release/actor2.wasm".to_string(),
        "actor2".to_string(),
    );
    actor2.start().await?;

    Ok(())
}
