#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use tokio::runtime::Builder;

risc0_zkvm::guest::entry!(main);

async fn perform_async_task() {
    6u32;
}

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        perform_async_task().await;
    });


    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
