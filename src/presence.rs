extern crate discord_rpc_client;

use discord_rpc_client::Client;
use std::thread;


pub const ID: u64 = 886460989040652350;  

pub fn start_connection(status: &String) {
    let state_message = status;
    let details_msg = "Rusty";

    let mut drpc = Client::new(ID);


    let state_message = state_message.clone();

    
    drpc.start();
    
    drpc.set_activity(|act| act.state(state_message)
                               .details(details_msg)
    ).expect("Failed to set activity");

    loop { /* And then we wait */  }
    
}



/*
pub fn start_connection(status: &String) {
    let state_message = status;
    let details_msg = "Rusty";

    let mut drpc = Client::new(ID);


    let state_message = state_message.clone();
    eprintln!("{}", state_message);

    thread::spawn(move || {
        drpc.start();
        eprintln!("starts");
        drpc.set_activity(|act| act.state(state_message)
                                   .details(details_msg)
        ).expect("Failed to set activity");

        eprintln!("Sets");
        loop {
            eprintln!("Loops");
        }
    });

    
}

*/


/*
eprintln!("Function Ran!");

    let mut client = Client::new(ID);
    
    eprintln!("Created Client");
    client.start();
    
    eprintln!("Started");

    client.set_activity(|act| act.state(status)).expect("Failed to set status");
    eprintln!("Created Activity");

    loop {
        eprintln!("Looping thru this bad boy")
    }
*/
