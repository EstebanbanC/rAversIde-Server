// main.rs
#[macro_use] extern crate rocket;

mod chatbot;
mod rename;
mod utils;
mod analyze;
mod test;
use futures::SinkExt;
use rocket::launch;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};
use tokio::net::{TcpListener, TcpStream};
use crate::chatbot::WsChannel;

async fn handle_connection(mut ws_stream: WebSocketStream<TcpStream>, mut rx: broadcast::Receiver<String>) {
    println!("handle connection");

    // Envoie un message initial au client
    ws_stream.send(Message::Text("Connected to server\n".to_string())).await.expect("Failed to send WebSocket message");

    while let Ok(message) = rx.recv().await {
        println!("Received a message from the channel: {}", message);
        ws_stream.send(Message::Text(message)).await.expect("Failed to send WebSocket message");
    }
}

// Endpoint pour la page d'accueil et test
#[get("/")]
pub async fn index() -> &'static str {
    // if let Err(e) = ask_chat_gpt_chatbot("Hello!".to_string()).await {
    //     eprintln!("Erreur lors de l'appel de ask_chat_gpt_chatbot: {}", e);
    // }
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (tx, _) = broadcast::channel::<String>(32);
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            println!("New WebSocket connection: {}", stream.peer_addr().unwrap());
            let ws_stream = accept_async(stream).await.expect("Error during the WebSocket handshake");

            // Abonnez-vous au canal de diffusion pour cette connexion
            let rx = tx.subscribe();
            tokio::spawn(handle_connection(ws_stream, rx));
        }
    });


    rocket::build()
        .manage(WsChannel::new(tx_clone))
        .mount("/", routes![
            rename::rename_function, 
            rename::rename_variable,
            chatbot::handle_chatbot,
            analyze::analyze,
            index
        ])
}