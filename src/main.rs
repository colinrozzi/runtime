use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <wasm_file> <port>", args[0]);
        std::process::exit(1);
    }

    let wasm_path = &args[1];
    let port: u16 = args[2].parse()?;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let runtime = runtime::Runtime::new(wasm_path)?;
    runtime.start(addr).await?;

    Ok(())
}
