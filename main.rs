use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::net::tcp::TcpListener;
use std::io::IoError;
use std::io::{Listener, Acceptor};
use std::io::EndOfFile;

fn main() {
	let ipv4 = Ipv4Addr(127, 0, 0 , 1);
	let addr = SocketAddr { ip: ipv4, port: 7 };
	let res_listener = TcpListener::bind(addr);

	if res_listener.is_err() {
		println!("Error while binding listener"); // Don't forget to give root permissions for ports lower than 1024
		printError(res_listener.err().unwrap());
		return;
	}
	
	let listener = res_listener.ok().unwrap();
	let res_acceptor = listener.listen();
	
	if res_acceptor.is_err() {
		println!("Error while starting listener");
		printError(res_acceptor.err().unwrap());
		return;
	}

	let mut acceptor = res_acceptor.ok().unwrap();

	for res_stream in acceptor.incoming() { // .incoming blocks till the next connection
		spawn(proc() { // We spawn a new task to allow for multiple clients at once
			if res_stream.is_err() {
				println!("Error while accepting connection");
				printError(res_stream.err().unwrap());
				return;
			}
	
			let mut stream = res_stream.ok().unwrap();
			
			loop {
				let mut data = ~[0u8, .. 128];
				let res_data_amount = stream.read(data);
				
				if res_data_amount.is_err() {
					let error = res_data_amount.err().unwrap();
					if error.kind != EndOfFile { // We don't want to print an error every time a socket is closed
						println!("Error while reading from connection");
						printError(error);
						return;
					}

					break; // Connection Closed
				}

				let data_amount = res_data_amount.ok().unwrap();
				let write_res = stream.write(data.slice(0, data_amount));
	
				if write_res.is_err() {
					println!("Error while writing to connection");
					printError(write_res.err().unwrap());
				}
				// We don't need to do anything if writing actually succeeded - the Ok is empty
			}
		});
	}
}

// This function will print errors of type 'IoError' out
fn printError(error : IoError) {
	println!("Error Kind: {:?}", error.kind);
	println!("Description: {}", error.desc);
	println!("Details: {}", error.detail);
}
