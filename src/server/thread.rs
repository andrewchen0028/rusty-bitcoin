use std::{
  io::{Read, Result},
  net::TcpListener,
};

use crate::util::{constants::MSG_SIZE, networking::addr_from_node_number};

/* [TODO & NOTES]
 TODO:
 - Initialize TCP listener and bind to address.
 - Open listener to accept incoming streams.
 - Print each stream to stdout.

 NOTES:
 - For each incoming stream, BUT only run when there is an incoming stream.
 - Try with synchronous TcpListener first. Switch to async_std if necessary.

*/

/// Start listening for messages on TCP listener.
pub fn start_server_thread(node_number: &u8) -> Result<()> {
  // Get address from provided node number.
  let addr = addr_from_node_number(*node_number);

  // Initialize listener and bind to address.
  let listener = TcpListener::bind(addr)?;

  // Listen for and print messages until SHUTDOWN message is received.
  println!("Listening for streams...");
  for stream_result in listener.incoming() {
    // Try to extract TcpStream from stream_result.
    let mut stream = stream_result?;

    // Read stream to buffer.
    let mut buf = [0u8; MSG_SIZE];
    stream.read_exact(&mut buf)?;

    // Print buffer.
    println!(
      "Server at {} received message {:?} from client at {}",
      &addr,
      buf,
      stream.peer_addr()?,
    );
  }
  println!("Done listening for streams.");

  /* [TODO: TRY THIS IF ^THAT^ DOESN'T WORK]
  // Make `optional` of type `Option<i32>`
  let mut optional = Some(0);

  // This reads: "while `let` destructures `optional` into
  // `Some(i)`, evaluate the block (`{}`). Else `break`.
  while let Some(i) = optional {
    if i > 9 {
      println!("Greater than 9, quit!");
      optional = None;
    } else {
      println!("`i` is `{:?}`. Try again.", i);
      optional = Some(i + 1);
    }
    // ^ Less rightward drift and doesn't require
    // explicitly handling the failing case.
  }
  // ^ `if let` had additional optional `else`/`else if`
  // clauses. `while let` does not have these.
  */

  Ok(())
}
