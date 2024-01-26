use matrix_sdk::{ruma::RoomId, Client};

use crate::{pages::chat::chat::MessageItem, services::matrix::matrix::join_room};

pub async fn handle_command(message_item: &MessageItem, client: &Client) {
    let query: Vec<String> = message_item
        .msg
        .trim()
        .split(' ')
        .map(|val| val.to_string())
        .collect();

    let action = query[0].as_str();
    let rid = query[1].clone();

    let room_id = RoomId::parse(rid);

    match action {
        "!join" => match room_id {
            Ok(room_id) => {
                join_room(client, &room_id).await;
            }
            Err(_) => {}
        },
        _ => {}
    };
}
