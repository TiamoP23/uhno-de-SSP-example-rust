use std::time::Duration;

use dotenv::dotenv;
use modals::GameEvent;
use rust_socketio::{ClientBuilder, Event, Payload, RawClient, TransportType};
use serde_json::json;

use crate::modals::GameMove;

mod modals;

fn main() {
    dotenv().ok();

    let gameserver = std::env::var("GAMESERVER").expect("GAMESERVER not set");

    let _socket = ClientBuilder::new(gameserver)
        .transport_type(TransportType::Websocket)
        .on(Event::Error, handle_error)
        .on(Event::Connect, handle_connect)
        .on(Event::Close, handle_close)
        .on(Event::Custom(String::from("data")), handle_data)
        .connect()
        .expect("Connection failed");

    loop {}
}

fn handle_connect(_: Payload, client: RawClient, _: Option<i32>) {
    println!("Connected to server!");

    authenticate(client);
}

fn handle_close(_: Payload, _: RawClient, _: Option<i32>) {
    println!("Connection closed");
}

fn handle_error(payload: Payload, _: RawClient, _: Option<i32>) {
    println!("Error: {:#?}", payload);
}

fn handle_data(payload: Payload, client: RawClient, packet_id: Option<i32>) {
    let payload = match payload {
        Payload::String(data) => data,
        _ => {
            println!("Payload is not a string");
            return;
        }
    };

    let event = match serde_json::from_str::<GameEvent>(&payload) {
        Ok(event) => event,
        Err(err) => {
            println!("Parsing Error: {:#?}", err);
            return;
        }
    };

    handle_game_event(event, client, packet_id);
}

fn handle_game_event(event: GameEvent, client: RawClient, packet_id: Option<i32>) {
    match event {
        GameEvent::Init(event) => {
            println!("New Game {:#?}", event.details.id);
        }
        GameEvent::Round(_) => {
            let game_move: GameMove = rand::random();

            println!("Sending move: {:#?}", game_move);

            client
                .emit_ack(packet_id, serde_json::to_string(&game_move).unwrap())
                .expect("Server unreachable");
        }
        GameEvent::Result(event) => {
            println!("Game {:#?} finished", event.details.id);
            println!("Result: {:#?}", event.details.players);
        }
    }
}

fn authenticate(client: RawClient) {
    let secret = std::env::var("SECRET").expect("SECRET not set");

    client
        .emit_with_ack(
            "authenticate",
            json!(secret),
            Duration::from_secs(2),
            |payload, _, _| {
                let payload = match payload {
                    Payload::String(data) => data,
                    _ => {
                        println!("Payload is not a string");
                        return;
                    }
                };

                let success = match serde_json::from_str::<(bool,)>(&payload) {
                    Ok((success,)) => success,
                    Err(err) => {
                        println!("Parsing Error: {:#?}", err);
                        return;
                    }
                };

                if success {
                    println!("Authenticated successfully");
                } else {
                    println!("Authentication failed");
                }
            },
        )
        .expect("Server unreachable");
}
