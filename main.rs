
use crossbeam_channel::{bounded, Receiver, Sender};
use std::thread;

struct IncrementRequest {
    number: u32,
    response_channel: Sender<u32>,
}

fn client_process(server_channels: Vec<Sender<IncrementRequest>>, number: u32) {
    let (response_sender, response_receiver): (Sender<u32>, Receiver<u32>) = bounded(1);

    for server_channel in server_channels {
        server_channel.send(IncrementRequest {
            number,
            response_channel: response_sender.clone(),
        });
    }

    let mut results = Vec::new();
    for _ in 0..server_channels.len() {
        let result = response_receiver.recv().unwrap();
        results.push(result);
    }

    println!("Results: {:?}", results);
}

fn server_process(server_channel: Receiver<IncrementRequest>) {
    for increment_request in server_channel {
        let result = increment_request.number + 1;
        increment_request.response_channel.send(result);
    }
}

fn main() {
    let num_servers = 3;
    let mut server_channels = Vec::new();

    for _ in 0..num_servers {
        let (server_sender, server_receiver): (Sender<IncrementRequest>, Receiver<IncrementRequest>) = bounded(1);
        server_channels.push(server_sender);

        thread::spawn(move || server_process(server_receiver));
    }

    let number = 5;
    client_process(server_channels, number);
    // print!("HELLO");
}


