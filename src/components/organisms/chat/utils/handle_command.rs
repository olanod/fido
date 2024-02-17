use matrix_sdk::{ruma::RoomId, Client};
use ruma::OwnedRoomId;

use crate::{pages::chat::chat::MessageItem, services::matrix::matrix::join_room};

pub enum CommandError {
    RoomIdNotFound,
    ActionNotFound,
    InvalidRoomId,
    RequestFailed,
}

pub async fn handle_command(
    message_item: &MessageItem,
    client: &Client,
) -> Result<OwnedRoomId, CommandError> {
    let query: Vec<String> = message_item
        .msg
        .trim()
        .split(' ')
        .map(|val| val.to_string())
        .collect();

    let action = query.get(0).ok_or(CommandError::ActionNotFound)?.as_str();
    let rid = query.get(1).ok_or(CommandError::InvalidRoomId)?;

    let room_id = RoomId::parse(rid).map_err(|_| CommandError::InvalidRoomId)?;

    match action {
        "!join" => join_room(client, &room_id)
            .await
            .map_err(|_| CommandError::RequestFailed),
        _ => Err(CommandError::ActionNotFound),
    }
}
