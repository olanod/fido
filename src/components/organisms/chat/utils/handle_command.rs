use crate::{pages::chat::chat::MessageItem, services::matrix::matrix::join_room};
use matrix_sdk::{ruma::RoomId, Client};

pub enum CommandError {
    RoomIdNotFound,
    ActionNotFound,
    InvalidRoomId,
    RequestFailed,
}

pub enum Command {
    Join(String),
    PublicRooms,
}

pub async fn handle_command(
    message_item: &MessageItem,
    client: &Client,
) -> Result<Command, CommandError> {
    let query: Vec<String> = message_item
        .msg
        .trim()
        .split(' ')
        .map(|val| val.to_string())
        .collect();

    let action = query.get(0).ok_or(CommandError::ActionNotFound)?.as_str();

    match action {
        "!join" => {
            let rid = query.get(1).ok_or(CommandError::InvalidRoomId)?;
            let room_id = RoomId::parse(rid).map_err(|_| CommandError::InvalidRoomId)?;
            join_room(client, &room_id)
                .await
                .map(|id| Command::Join(id.to_string()))
                .map_err(|_| CommandError::RequestFailed)
        }
        "!rooms" => Ok(Command::PublicRooms),
        _ => Err(CommandError::ActionNotFound),
    }
}
