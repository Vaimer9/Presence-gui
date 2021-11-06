extern crate discord_rpc_client;

use discord_rpc_client::Client;

pub const ID: u64 = 886460989040652350;  

pub fn start_connection(status: &String, should_stay: bool) {
    let mut client = Client::new(ID);
    client.start();

    client.set_activity(|act| act.state(status)).expect("Failed to set status");

    while should_stay {} 

}
