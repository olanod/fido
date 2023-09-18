use matrix_sdk::{ruma::RoomId, Client};

use crate::{services::matrix::matrix::join_room, pages::chat::chat::MessageItem};

pub async fn handle_command(message_item: MessageItem, client: &Client) {
    let query: Vec<String> = message_item
        .msg
        .trim()
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();

    let action = query[0].as_str();
    let rid = query[1].clone();

    let room_id = RoomId::parse(rid).unwrap();

    match action {
        "!join" => join_room(client, &room_id).await,
        _ => {}
    };
}
