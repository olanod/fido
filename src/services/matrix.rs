pub mod matrix {
    use log::info;

    use matrix_sdk::{
        self,
        attachment::AttachmentConfig,
        config::SyncSettings,
        deserialized_responses::{SyncTimelineEvent, TimelineSlice},
        room::{MessagesOptions, Room},
        ruma::{
            api::{
                client::filter::{LazyLoadOptions, RoomEventFilter},
                error::{FromHttpResponseError, ServerError},
            },
            assign,
            events::{
                room::{
                    message::{
                        InReplyTo, MessageType, OriginalSyncRoomMessageEvent, Relation,
                        RoomMessageEventContent,
                    },
                    MediaSource,
                },
                AnyMessageLikeEvent, AnySyncMessageLikeEvent, AnySyncTimelineEvent,
                AnyTimelineEvent, MessageLikeEvent, SyncMessageLikeEvent,
            },
            OwnedEventId, OwnedUserId, RoomId, TransactionId, UInt,
        },
        Client, Error, HttpError,
    };
    use url::Url;

    use crate::{
        components::atoms::room::RoomItem,
        utils::matrix::{mxc_to_https_uri, ImageSize},
    };

    use matrix_sdk::ruma::exports::serde_json;
    use matrix_sdk::Session;

    use serde::{Deserialize, Serialize};

    // #[derive(Sized)]
    pub struct Attachment {
        pub(crate) data: Vec<u8>,
    }

    pub async fn create_client(homeserver_url_str: String) -> Client {
        info!("create client ");
        let homeserver_url =
            Url::parse(&homeserver_url_str).expect("Couldn't parse the homeserver URL");
        let client = Client::new(homeserver_url).await.unwrap();

        client
    }

    pub async fn login_and_sync(
        client: &Client,
        username: String,
        password: String,
    ) -> Result<String, String> {
        info!("Logging");
        let response = client
            .login_username(&username, &password)
            .initial_device_display_name("rust-sdk")
            .send()
            .await;

        match response {
            Ok(res) => {
                info!("res: {:?}", res);
                info!("Syncing");
                client.sync_once(SyncSettings::default()).await.unwrap();

                return Ok(String::from("Welcome"));
            }
            Err(err) => match err {
                Error::Http(HttpError::Api(FromHttpResponseError::Server(ServerError::Known(
                    matrix_sdk::RumaApiError::ClientApi(m),
                )))) => return Err(m.message),
                _ => return Err(String::from("An error have been ocurred!")),
            },
        }
    }

    pub async fn join_room(client: &Client, room_id: &RoomId) {
        info!("Joining room");
        client.join_room_by_id(&room_id).await.unwrap();
    }

    pub async fn send_message(
        client: &Client,
        room_id: &RoomId,
        msg: String,
        reply_to: Option<OwnedEventId>,
    ) {
        let room = client.get_joined_room(&room_id).unwrap();
        let tx_id = TransactionId::new();

        info!("Sending message");
        let mut x = RoomMessageEventContent::text_plain(msg);

        if let Some(r) = reply_to {
            x.relates_to = Some(matrix_sdk::ruma::events::room::message::Relation::Reply {
                in_reply_to: InReplyTo::new(r),
            });
        }

        room.send(x, Some(&tx_id)).await.unwrap();
    }

    pub async fn send_attachment(client: &Client, room_id: &RoomId, attach: &Attachment) {
        let room = client.get_joined_room(&room_id).unwrap();

        info!("Sending message");

        room.send_attachment(
            "asdf",
            &mime::IMAGE_PNG,
            &attach.data,
            AttachmentConfig::new(),
        )
        .await
        .unwrap();
    }

    pub fn listen_messages(client: &Client) {
        client.add_event_handler(|ev: OriginalSyncRoomMessageEvent| async move {
            info!("Received event {}: {:?}", ev.sender, ev.content.body());
        });
    }

    pub fn list_rooms(client: &Client) -> Vec<RoomItem> {
        let mut rooms = Vec::new();
        let x = client.joined_rooms();

        info!("{x:?}");
        for room in client.rooms() {
            let x = room.avatar_url();

            if let Some(name) = room.name() {
                let avatar_uri: Option<String> = match x {
                    Some(avatar) => {
                        let (server, id) = avatar.parts().unwrap();
                        let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width=24&height=24&method=crop", server, id);
                        Some(String::from(uri))
                    }
                    None => None,
                };

                rooms.push(RoomItem {
                    avatar_uri: avatar_uri,
                    id: room.room_id().to_string(),
                    name: name,
                })
            }
        }

        rooms
    }

    #[derive(Debug, Clone)]
    pub struct RoomMember {
        pub id: String,
        pub name: String,
        pub avatar_uri: Option<String>,
    }

    pub async fn room_member(member: OwnedUserId, room: &Room) -> RoomMember {
        let member = room.get_member(&member).await;

        match member {
            Ok(rm) => match rm {
                Some(m) => {
                    let avatar = m.avatar_url();

                    let avatar_uri: Option<String> = match avatar {
                        Some(avatar) => {
                            let (server, id) = avatar.parts().unwrap();
                            let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width=24&height=24&method=crop", server, id);
                            Some(String::from(uri))
                        }
                        None => None,
                    };

                    match m.display_name() {
                        Some(name) => RoomMember {
                            id: String::from(m.user_id()),
                            name: String::from(name),
                            avatar_uri,
                        },
                        _ => panic!("Member not found"),
                    }
                }
                _ => panic!("Member not found"),
            },
            Err(_) => {
                panic!("Member not found")
            }
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum TimelineMessageType {
        Image(String),
        Text(String),
    }

    #[derive(Debug, Clone)]
    pub struct TimelineMessageEvent {
        pub event_id: Option<String>,
        pub sender: RoomMember,
        pub body: TimelineMessageType,
        pub reply: Option<Box<TimelineMessageEvent>>,
    }

    pub async fn timeline(
        client: &Client,
        room_id: &RoomId,
        limit: u64,
    ) -> Vec<TimelineMessageEvent> {
        let mut messages: Vec<TimelineMessageEvent> = Vec::new();

        let room = client.get_room(&room_id).unwrap();

        let filter = assign!(RoomEventFilter::default(), {
            lazy_load_options: LazyLoadOptions::Enabled { include_redundant_members: false },
        });
        let options = assign!(MessagesOptions::backward(), {
            limit: UInt::new(limit).unwrap(),
            filter,
        });
        let m = room.messages(options).await.unwrap();

        let t = TimelineSlice::new(
            m.chunk.into_iter().map(SyncTimelineEvent::from).collect(),
            m.start,
            m.end,
            false,
            false,
        );

        for zz in t.events.iter() {
            let deserialized =
                deserialize_any_timeline_event(zz.event.deserialize().unwrap(), &room).await;

            if let Some(d) = deserialized {
                messages.push(d);
            }
        }

        messages
    }

    pub async fn deserialize_any_timeline_event(
        ev: AnySyncTimelineEvent,
        room: &Room,
    ) -> Option<TimelineMessageEvent> {
        match ev {
            AnySyncTimelineEvent::MessageLike(AnySyncMessageLikeEvent::RoomMessage(
                SyncMessageLikeEvent::Original(original),
            )) => {
                let n = &original.content.msgtype;
                let event = original.event_id;

                let member = room_member(original.sender, &room).await;
                let relates = &original.content.relates_to;

                let message_result =
                    format_original_any_room_message_event(&n, event, &member).await;
                let message_result =
                    format_reply_from_event(&n, relates, &room, message_result, &member).await;
                message_result
            }
            _ => None,
        }
    }

    pub async fn deserialize_timeline_event(
        ev: AnyTimelineEvent,
        room: &Room,
    ) -> Option<TimelineMessageEvent> {
        match ev {
            AnyTimelineEvent::MessageLike(AnyMessageLikeEvent::RoomMessage(
                MessageLikeEvent::Original(original),
            )) => {
                let n = &original.content.msgtype;
                let member = room_member(original.sender, &room).await;
                let event = original.event_id;

                let message_result =
                    format_original_any_room_message_event(&n, event, &member).await;

                message_result
            }
            _ => None,
        }
    }

    pub async fn format_original_any_room_message_event(
        n: &MessageType,
        event: OwnedEventId,
        member: &RoomMember,
    ) -> Option<TimelineMessageEvent> {
        let mut message_result = None;

        match &n {
            MessageType::Image(nm) => match &nm.source {
                MediaSource::Plain(mx_uri) => {
                    let https_uri = mxc_to_https_uri(
                        &mx_uri,
                        ImageSize {
                            width: 800,
                            height: 600,
                        },
                    );

                    if let Some(uri) = https_uri {
                        message_result = Some(TimelineMessageEvent {
                            event_id: Some(String::from(event.as_str())),
                            reply: None,
                            sender: member.clone(),
                            body: TimelineMessageType::Image(uri),
                        });
                    }
                }
                MediaSource::Encrypted(_) => {
                    panic!("Unsupporterd encrypted image");
                }
            },
            MessageType::Text(content) => {
                message_result = Some(TimelineMessageEvent {
                    event_id: Some(String::from(event.as_str())),
                    sender: member.clone(),
                    body: TimelineMessageType::Text(content.body.clone()),
                    reply: None,
                });
            }
            _ => {}
        }

        return message_result;
    }

    pub async fn format_reply_from_event(
        n: &MessageType,
        relates: &Option<Relation>,
        room: &Room,
        message_result: Option<TimelineMessageEvent>,
        member: &RoomMember,
    ) -> Option<TimelineMessageEvent> {
        let mut message_result: Option<TimelineMessageEvent> = message_result;

        match relates {
            Some(r) => match r {
                matrix_sdk::ruma::events::room::message::Relation::Reply { in_reply_to } => {
                    let room_event = room.event(&in_reply_to.event_id).await;

                    match room_event {
                        Ok(event) => {
                            let desc_event = event.event.deserialize().unwrap();

                            let reply = deserialize_timeline_event(desc_event, room).await;

                            match reply {
                                Some(r) => match &r.body {
                                    TimelineMessageType::Image(_uri) => {
                                        if let Some(mut mr) = message_result {
                                            mr.reply = Some(Box::from(r.clone()));
                                            message_result = Some(mr)
                                        }

                                        if n.body().contains("sent an image.") {
                                            let to_remove =
                                                format!("> <{}> {}", r.sender.id, "sent an image.");

                                            let uncleared_content = n.body();
                                            let n =
                                                uncleared_content.replace(&to_remove, "").clone();

                                            let content_body = TimelineMessageType::Text(n);

                                            message_result = Some(TimelineMessageEvent {
                                                event_id: None,
                                                sender: member.clone(),
                                                body: content_body,
                                                reply: Some(Box::from(r)),
                                            });
                                        }
                                    }
                                    TimelineMessageType::Text(body) => {
                                        let to_remove =
                                            format!("> <{}> {}", r.sender.id, body.trim());

                                        let uncleared_content = n.body();
                                        let n = uncleared_content.replace(&to_remove, "").clone();

                                        let content_body = TimelineMessageType::Text(n);

                                        message_result = Some(TimelineMessageEvent {
                                            event_id: None,
                                            sender: member.clone(),
                                            body: content_body,
                                            reply: Some(Box::from(r)),
                                        });
                                    }
                                },
                                _ => return None,
                            }
                        }
                        Err(_) => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }

        return message_result;
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClientSession {
        pub homeserver: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FullSession {
        pub client_session: ClientSession,
        pub user_session: Session,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub sync_token: Option<String>,
    }

    pub struct LoginResult {
        // client
    }

    pub async fn login(
        homeserver: String,
        username: String,
        password: String,
    ) -> anyhow::Result<(Client, String)> {
        info!("No previous session found, logging in…");

        let (client, client_session) = build_client(homeserver).await?;

        match client
            .login_username(&username, &password)
            .initial_device_display_name("persist-session client")
            .send()
            .await
        {
            Ok(_) => {
                info!("Logged in as {username}");
            }
            Err(error) => {
                info!("Error logging in: {error}");
                match error {
                    // Error::Http(HttpError::Api(FromHttpResponseError::Server(
                    //     ServerError::Known(matrix_sdk::RumaApiError::ClientApi(m)),
                    // ))) => return Err(m.into()),
                    _ => return Err(error.into()),
                }
            }
        }

        let user_session = client
            .session()
            .expect("A logged-in client should have a session");

        let serialized_session = serde_json::to_string(&FullSession {
            client_session,
            user_session,
            sync_token: None,
        })?;

        info!("Syncing");
        // client.sync_once(SyncSettings::default()).await.unwrap();

        Ok((client, serialized_session))
    }

    pub async fn restore_session(
        serialized_session: &str,
    ) -> anyhow::Result<(Client, Option<String>)> {
        info!("Previous session found in session_file",);

        let FullSession {
            client_session,
            user_session,
            sync_token,
        } = serde_json::from_str(&serialized_session)?;

        let client = Client::builder()
            .homeserver_url(client_session.homeserver.clone())
            .indexeddb_store("b", None)
            .await?;

        let client = client.build().await?;

        info!("Restoring session for {}…", user_session.user_id);

        client.restore_login(user_session).await?;

        Ok((client, sync_token))
    }

    pub async fn build_client(homeserver: String) -> anyhow::Result<(Client, ClientSession)> {
        loop {
            match Client::builder()
                .homeserver_url(&homeserver)
                .indexeddb_store("b", None)
                .await
            {
                Ok(builder) => match builder.build().await {
                    Ok(client) => return Ok((client, ClientSession { homeserver })),
                    Err(error) => match &error {
                        matrix_sdk::ClientBuildError::AutoDiscovery(_)
                        | matrix_sdk::ClientBuildError::Url(_)
                        | matrix_sdk::ClientBuildError::Http(_) => {
                            info!("{error}");
                            return Err(error.into());
                        }
                        _ => {
                            return Err(error.into());
                        }
                    },
                },
                Err(err) => {
                    info!("err {}", err)
                }
            }
        }
    }
}
