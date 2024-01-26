pub mod matrix {
    use std::{
        collections::HashMap,
        io::{self, ErrorKind},
        ops::Deref,
        time::{Duration, UNIX_EPOCH},
    };

    use chrono::{DateTime, Local, Utc};
    use log::info;

    use matrix_sdk::{
        self,
        attachment::AttachmentConfig,
        config::SyncSettings,
        deserialized_responses::{SyncTimelineEvent, TimelineSlice},
        media::{MediaFormat, MediaRequest, MediaThumbnailSize},
        room::{MessagesOptions, Room},
        ruma::{
            api::{
                self,
                client::{
                    filter::{LazyLoadOptions, RoomEventFilter},
                    media::get_content_thumbnail::v3::Method,
                    room::{create_room::v3::RoomPreset, Visibility},
                    uiaa,
                },
                error::{FromHttpResponseError, ServerError},
            },
            assign,
            events::{
                room::{
                    avatar::RoomAvatarEventContent,
                    message::{
                        FileMessageEventContent, ImageMessageEventContent, InReplyTo,
                        MessageFormat, MessageType, OriginalSyncRoomMessageEvent, Relation,
                        RoomMessageEventContent, VideoMessageEventContent,
                    },
                    MediaSource,
                },
                AnyInitialStateEvent, AnyMessageLikeEvent, AnySyncMessageLikeEvent,
                AnySyncTimelineEvent, AnyTimelineEvent, EmptyStateKey, InitialStateEvent,
                MessageLikeEvent, SyncMessageLikeEvent,
            },
            serde::Raw,
            MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedUserId, RoomId, TransactionId, UInt,
        },
        Client, Error, HttpError, HttpResult,
    };
    use mime::Mime;
    use ruma::{
        api::client::message::send_message_event::v3::Response, events::room::message::Thread,
        OwnedMxcUri, UserId,
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
        pub body: String,
        pub(crate) data: Vec<u8>,
        pub content_type: Mime,
    }

    pub struct AttachmentStream {
        pub attachment: Attachment,
        pub send_to_thread: bool,
    }

    pub async fn create_client(homeserver_url_str: String) -> Client {
        info!("create client ");
        let homeserver_url =
            Url::parse(&homeserver_url_str).expect("Couldn't parse the homeserver URL");
        let client = Client::new(homeserver_url)
            .await
            .expect("can't handle new Client: create_client");

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
                client
                    .sync_once(SyncSettings::default())
                    .await
                    .expect("can't sync: login_and_sync");

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
        client
            .join_room_by_id(&room_id)
            .await
            .expect("can't join room: join_room");
    }

    pub async fn send_message(
        client: &Client,
        room_id: &RoomId,
        msg: MessageType,
        reply_to: Option<OwnedEventId>,
        thread_to: Option<OwnedEventId>,
        latest_event: Option<OwnedEventId>,
    ) -> Result<Response, Error> {
        let room = client
            .get_joined_room(&room_id)
            .expect("can't get joined room: send_message");
        let tx_id = TransactionId::new();

        let mut event_content = RoomMessageEventContent::new(msg);

        match reply_to {
            Some(r) => {
                event_content.relates_to =
                    Some(matrix_sdk::ruma::events::room::message::Relation::Reply {
                        in_reply_to: InReplyTo::new(r.clone()),
                    });

                if let Some(t) = &thread_to {
                    let thread = Thread::reply(t.clone(), r);

                    event_content.relates_to = Some(
                        matrix_sdk::ruma::events::room::message::Relation::Thread(thread),
                    );
                }
            }
            None => {}
        }

        match latest_event {
            Some(l) => {
                if let Some(t) = &thread_to {
                    let thread = Thread::plain(t.clone(), l);

                    event_content.relates_to = Some(
                        matrix_sdk::ruma::events::room::message::Relation::Thread(thread),
                    );
                }
            }
            None => {}
        }

        room.send(event_content, Some(&tx_id)).await
    }

    pub async fn upload_attachment(
        client: &Client,
        attach: &Attachment,
    ) -> Result<ruma::api::client::media::create_content::v3::Response, Error> {
        client
            .media()
            .upload(&attach.content_type, &attach.data)
            .await
    }

    pub async fn send_attachment(
        client: &Client,
        room_id: &RoomId,
        uri: &OwnedMxcUri,
        attach: &Attachment,
        reply_to: Option<OwnedEventId>,
        thread_to: Option<OwnedEventId>,
        latest_event: Option<OwnedEventId>,
    ) -> Result<Response, Error> {
        let room = client
            .get_joined_room(&room_id)
            .expect("can't get joined room: send_attachment");

        let message_type = match attach.content_type.type_() {
            mime::IMAGE => {
                let event_content =
                    ImageMessageEventContent::plain(attach.body.clone(), uri.clone(), None);

                MessageType::Image(event_content)
            }
            mime::VIDEO => {
                let event_content =
                    VideoMessageEventContent::plain(attach.body.clone(), uri.clone(), None);

                MessageType::Video(event_content)
            }
            mime::APPLICATION => {
                let event_content =
                    FileMessageEventContent::plain(attach.body.clone(), uri.clone(), None);

                MessageType::File(event_content)
            }
            _ => {
                let error = io::Error::new(ErrorKind::Other, "Error al subir el archivo");
                return Err(Error::Io(error));
            }
        };

        let response = if reply_to.is_some() || latest_event.is_some() {
            send_message(
                client,
                room_id,
                message_type,
                reply_to,
                thread_to,
                latest_event,
            )
            .await
        } else {
            room.send_attachment(
                &attach.body,
                &attach.content_type,
                &attach.data,
                AttachmentConfig::new(),
            )
            .await
        };

        response
    }

    pub fn listen_messages(client: &Client) {
        client.add_event_handler(|ev: OriginalSyncRoomMessageEvent| async move {
            info!("Received event {}: {:?}", ev.sender, ev.content.body());
        });
    }

    pub struct Conversations {
        pub rooms: Vec<RoomItem>,
        pub spaces: HashMap<RoomItem, Vec<RoomItem>>,
    }

    pub async fn list_rooms_and_spaces(client: &Client) -> Conversations {
        let mut rooms = Vec::new();
        let mut spaces = HashMap::new();

        for room in client.joined_rooms() {
            let is_direct = room.is_direct();
            let is_space = room.is_space();

            let avatar_url = room.avatar_url();

            let avatar_uri: Option<String> = match avatar_url {
                Some(avatar) => {
                    let (server, id) = avatar.parts().unwrap();
                    let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width=48&height=48&method=crop", server, id);
                    Some(String::from(uri))
                }
                None => None,
            };

            if let Some(name) = room.name() {
                let room = RoomItem {
                    avatar_uri: avatar_uri,
                    id: room.room_id().to_string(),
                    name: name,
                    is_public: room.is_public(),
                    is_direct,
                };

                if is_space {
                    spaces.insert(room, vec![]);
                } else {
                    rooms.push(room);
                }
            } else {
                let me = client.whoami().await.unwrap();
                let users = room.members().await;

                if let Ok(members) = users {
                    let member = members
                        .into_iter()
                        .find(|member| !member.user_id().eq(&me.user_id));

                    if let Some(m) = member {
                        let name = m.name();

                        rooms.push(RoomItem {
                            avatar_uri: avatar_uri,
                            id: room.room_id().to_string(),
                            name: String::from(name),
                            is_public: room.is_public(),
                            is_direct,
                        })
                    }
                }
            }
        }

        let mut to_list_rooms = vec![];

        for (key, value) in spaces.iter_mut() {
            rooms.iter().for_each(|room| {
                let room_homeserver = room.id.split(":").collect::<Vec<&str>>()[1];
                let space_homeserver = key.id.split(":").collect::<Vec<&str>>()[1];

                if room_homeserver.eq(space_homeserver) && !room.is_direct && !room.id.eq(&key.id) {
                    value.push(room.clone());
                } else {
                    to_list_rooms.push(room.clone());
                }
            });
        }

        info!("final rooms {:#?}, ", to_list_rooms);
        info!("final spaces {:#?}, ", spaces);

        Conversations {
            rooms: to_list_rooms,
            spaces: spaces,
        }
    }

    #[derive(PartialEq, Debug, Clone)]
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
                            let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width=48&height=48&method=crop", server, id);
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

    #[derive(Clone)]
    pub struct AccountInfo {
        pub name: String,
        pub avatar_uri: Option<String>,
    }

    pub async fn account(client: &Client) -> AccountInfo {
        let avatar = client.account().get_avatar_url().await;
        let display_name = client.account().get_display_name().await;

        let avatar_uri = match avatar {
            Ok(uri) => {
                if let Some(avatar) = uri {
                    let avatar = &*avatar;
                    let (server, id) = avatar.parts().unwrap();
                    let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width=48&height=48&method=crop", server, id);

                    Some(String::from(uri).to_owned())
                } else {
                    None
                }
            }
            Err(_) => None,
        };

        let name = match display_name {
            Ok(name) => {
                if let Some(n) = name {
                    n
                } else {
                    String::from("")
                }
            }
            Err(_) => String::from(""),
        };

        AccountInfo { name, avatar_uri }
    }

    pub async fn create_room(
        client: &Client,
        is_dm: bool,
        users: &[OwnedUserId],
        name: Option<String>,
        avatar: Option<Vec<u8>>,
    ) -> HttpResult<api::client::room::create_room::v3::Response> {
        let mut request = api::client::room::create_room::v3::Request::new();

        let mut initstateevvec: Vec<Raw<AnyInitialStateEvent>> = vec![];

        if let Some(data) = avatar {
            let media_uri = client.media().upload(&mime::IMAGE_JPEG, &data).await;

            match media_uri {
                Ok(response) => {
                    let mut x = RoomAvatarEventContent::new();
                    x.url = Some(response.content_uri);

                    let initstateev: InitialStateEvent<RoomAvatarEventContent> =
                        InitialStateEvent {
                            content: x,
                            state_key: EmptyStateKey,
                        };

                    let rawinitstateev =
                        Raw::new(&initstateev).expect("can't create a new raw: create_room");

                    let rawanyinitstateev: Raw<AnyInitialStateEvent> = rawinitstateev.cast();
                    initstateevvec.push(rawanyinitstateev);
                    request.initial_state = &initstateevvec;
                }
                Err(_) => {}
            }
        }

        request.name = name.as_deref();
        request.is_direct = is_dm;

        let vis = Visibility::Private;
        if is_dm {
            request.invite = users;
            request.visibility = vis.clone();
            request.preset = Some(RoomPreset::PrivateChat);
        }

        client.create_room(request).await
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum ImageType {
        URL(String),
        Media(Vec<u8>),
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct FileContent {
        pub size: Option<u64>,
        pub body: String,
        pub source: Option<ImageType>,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum TimelineMessageType {
        Image(FileContent),
        Text(String),
        Html(String),
        File(FileContent),
        Video(FileContent),
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum EventOrigin {
        OTHER,
        ME,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct TimelineMessage {
        pub event_id: Option<String>,
        pub sender: RoomMember,
        pub body: TimelineMessageType,
        pub origin: EventOrigin,
        pub time: String,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct TimelineMessageReply {
        pub event: TimelineMessage,
        pub reply: Option<TimelineMessage>,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct TimelineMessageThread {
        pub event_id: String,
        pub thread: Vec<TimelineMessage>,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct TimelineThread {
        pub event_id: String,
        pub thread: Vec<TimelineMessage>,
        pub latest_event: String,
        pub count: usize,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum TimelineRelation {
        None(TimelineMessage),
        Reply(TimelineMessageReply),
        CustomThread(TimelineThread),
        Thread(TimelineMessageThread),
    }

    pub async fn timeline(
        client: &Client,
        room_id: &RoomId,
        limit: u64,
        from: Option<String>,
        old_messages: Vec<TimelineRelation>,
    ) -> (Option<String>, Vec<TimelineRelation>) {
        let mut messages: Vec<TimelineRelation> = old_messages;

        let room = client.get_room(&room_id).expect("can't get_room: timeline");

        let filter = assign!(RoomEventFilter::default(), {
            lazy_load_options: LazyLoadOptions::Enabled { include_redundant_members: false },
        });
        let options = assign!(MessagesOptions::backward(), {
            limit: UInt::new(20).expect("can't convert uint: timeline"),
            filter,
            from: from.as_deref()
        });

        let m = room
            .messages(options)
            .await
            .expect("can't get messages: timeline");

        info!("uncleared messagex matrix: {:#?}", m);

        let t = TimelineSlice::new(
            m.chunk.into_iter().map(SyncTimelineEvent::from).collect(),
            m.start,
            m.end.clone(),
            false,
            false,
        );

        let user = client.whoami().await;
        let mut me = String::from("");

        if let Ok(u) = user {
            me = u.user_id.to_string();
        }

        for zz in t.events.iter() {
            let deserialized = deserialize_any_timeline_event(
                zz.event
                    .deserialize()
                    .expect("can't deserialize iter events: timeline"),
                &room,
                &me,
                &client,
            )
            .await;

            if let Some(d) = deserialized {
                match &d {
                    TimelineRelation::Thread(x) => {
                        // Position of an existing thread timeline

                        let position = messages.iter().position(|m| {
                            if let TimelineRelation::CustomThread(y) = m {
                                y.event_id.eq(&x.event_id)
                            } else {
                                false
                            }
                        });

                        if let Some(p) = position {
                            if let TimelineRelation::CustomThread(ref mut z) = messages[p] {
                                z.thread.push(x.thread[0].clone());
                                z.thread.rotate_right(1);
                            };
                        } else {
                            let n = TimelineRelation::CustomThread(TimelineThread {
                                event_id: x.event_id.clone(),
                                thread: x.thread.clone(),
                                latest_event: x.thread[x.thread.len() - 1]
                                    .clone()
                                    .event_id
                                    .expect("can't get eventid from thread: timeline"),
                                count: x.thread.len(),
                            });

                            messages.push(n);
                            messages.rotate_right(1);
                        }
                    }
                    TimelineRelation::None(x) => {
                        // Position of a head thread timeline
                        let position = messages.iter().position(|m| {
                            if let TimelineRelation::CustomThread(y) = m {
                                y.event_id.eq(x
                                    .event_id
                                    .as_ref()
                                    .expect("can't compare event id: timeline"))
                            } else {
                                false
                            }
                        });

                        if let Some(p) = position {
                            if let TimelineRelation::CustomThread(ref mut z) = messages[p] {
                                let mm = format_head_thread(
                                    zz.event
                                        .deserialize()
                                        .expect("can't deserialize event custom thread: timeline"),
                                );

                                if let Some(x) = mm {
                                    z.latest_event = x.1;
                                }
                                z.thread.push(x.clone());
                                z.thread.rotate_right(1);
                            };
                        } else {
                            messages.push(d);
                            messages.rotate_right(1);
                        }
                    }
                    _ => {
                        messages.push(d);
                        messages.rotate_right(1);
                    }
                }
            }
        }

        (m.end, messages)
    }

    pub fn format_head_thread(ev: AnySyncTimelineEvent) -> Option<(usize, String)> {
        if let AnySyncTimelineEvent::MessageLike(AnySyncMessageLikeEvent::RoomMessage(
            SyncMessageLikeEvent::Original(original),
        )) = ev
        {
            if let Some(x) = original.unsigned.relations {
                if let Some(y) = x.thread {
                    Some((
                        2,
                        y.latest_event
                            .deserialize()
                            .expect("can't deserialize latest event: format_head_thread")
                            .event_id()
                            .to_string(),
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn deserialize_any_timeline_event(
        ev: AnySyncTimelineEvent,
        room: &Room,
        logged_user_id: &str,
        client: &Client,
    ) -> Option<TimelineRelation> {
        match ev {
            AnySyncTimelineEvent::MessageLike(AnySyncMessageLikeEvent::RoomMessage(
                SyncMessageLikeEvent::Original(original),
            )) => {
                let n = &original.content.msgtype;
                let event = original.event_id;

                if let Some(x) = original.unsigned.relations {
                    // x.thread.unwrap().latest_event
                }

                let member = room_member(original.sender, &room).await;
                let relates = &original.content.relates_to;
                let time = original.origin_server_ts;

                let formatted_message = format_original_any_room_message_event(
                    &n,
                    event,
                    &member,
                    &logged_user_id,
                    time,
                    &client,
                )
                .await;

                let mut message_result = None;

                match relates {
                    Some(relation) => {
                        match &relation {
                            Relation::_Custom => {
                                if let Some(x) = formatted_message {
                                    message_result = Some(TimelineRelation::None(x));
                                }
                            }

                            _ => {
                                if let Some(x) = formatted_message {
                                    message_result = format_relation_from_event(
                                        &n,
                                        relates,
                                        &room,
                                        x,
                                        &member,
                                        &logged_user_id,
                                        time,
                                        &client,
                                    )
                                    .await;
                                }
                            }
                        }

                        message_result
                    }
                    None => {
                        if let Some(x) = formatted_message {
                            message_result = Some(TimelineRelation::None(x));
                        }

                        message_result
                    }
                }
            }
            _ => None,
        }
    }

    pub async fn deserialize_timeline_event(
        ev: AnyTimelineEvent,
        room: &Room,
        logged_user_id: &str,
        client: &Client,
    ) -> Option<TimelineMessage> {
        match ev {
            AnyTimelineEvent::MessageLike(AnyMessageLikeEvent::RoomMessage(
                MessageLikeEvent::Original(original),
            )) => {
                let n = &original.content.msgtype;
                let member = room_member(original.sender, &room).await;
                let event = original.event_id;
                let time = original.origin_server_ts;

                let message_result = format_original_any_room_message_event(
                    &n,
                    event,
                    &member,
                    &logged_user_id,
                    time,
                    &client,
                )
                .await;

                message_result
            }
            _ => None,
        }
    }

    pub async fn format_original_any_room_message_event(
        n: &MessageType,
        event: OwnedEventId,
        member: &RoomMember,
        logged_user_id: &str,
        time: MilliSecondsSinceUnixEpoch,
        client: &Client,
    ) -> Option<TimelineMessage> {
        let mut message_result = None;

        let timestamp = {
            let d = UNIX_EPOCH + Duration::from_millis(time.0.into());

            let datetime = DateTime::<Local>::from(d);
            datetime.format("%H:%M").to_string()
        };

        match &n {
            MessageType::Image(nm) => match &nm.source {
                MediaSource::Plain(mx_uri) => {
                    let x = client
                        .media()
                        .get_media_content(
                            &MediaRequest {
                                source: nm.source.clone(),
                                format: MediaFormat::File,
                            },
                            true,
                        )
                        .await;

                    let https_uri = mxc_to_download_uri(&mx_uri);

                    let size = if let Some(file_info) = nm.info.clone() {
                        match file_info.size {
                            Some(size) => {
                                let size = size.to_string();
                                let size = size.parse::<u64>();

                                if let Ok(size) = size {
                                    Some(size)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };

                    if let Some(uri) = https_uri {
                        message_result = Some(TimelineMessage {
                            event_id: Some(String::from(event.as_str())),
                            sender: member.clone(),
                            body: TimelineMessageType::Image(FileContent {
                                size,
                                body: nm.body.clone(),
                                source: Some(ImageType::URL(uri)),
                            }),
                            origin: if member.id.eq(logged_user_id) {
                                EventOrigin::ME
                            } else {
                                EventOrigin::OTHER
                            },
                            time: timestamp,
                        });
                    }
                }
                MediaSource::Encrypted(file) => {
                    let x = client
                        .media()
                        .get_media_content(
                            &MediaRequest {
                                source: nm.source.clone(),
                                format: MediaFormat::Thumbnail(MediaThumbnailSize {
                                    method: Method::Crop,
                                    width: UInt::new(16).unwrap(),
                                    height: UInt::new(16).unwrap(),
                                }),
                            },
                            true,
                        )
                        .await;

                    let size = if let Some(file_info) = nm.info.clone() {
                        match file_info.size {
                            Some(size) => {
                                let size = size.to_string();
                                let size = size.parse::<u64>();

                                if let Ok(size) = size {
                                    Some(size)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };

                    if let Ok(content) = x {
                        message_result = Some(TimelineMessage {
                            event_id: Some(String::from(event.as_str())),
                            sender: member.clone(),
                            body: TimelineMessageType::Image(FileContent {
                                size,
                                body: nm.body.clone(),
                                source: Some(ImageType::Media(content)),
                            }),
                            origin: if member.id.eq(logged_user_id) {
                                EventOrigin::ME
                            } else {
                                EventOrigin::OTHER
                            },
                            time: timestamp,
                        });
                    }
                }
            },
            MessageType::Text(content) => {
                message_result = Some(TimelineMessage {
                    event_id: Some(String::from(event.as_str())),
                    sender: member.clone(),
                    body: TimelineMessageType::Text(content.body.clone()),
                    origin: if member.id.eq(logged_user_id) {
                        EventOrigin::ME
                    } else {
                        EventOrigin::OTHER
                    },
                    time: timestamp,
                });

                if let Some(formatted) = &content.formatted {
                    match formatted.format {
                        MessageFormat::Html => {
                            if let Some(ref mut x) = message_result {
                                x.body = TimelineMessageType::Html(formatted.body.clone());
                            }
                        }
                        _ => {}
                    }
                };
            }
            MessageType::File(f) => match &f.source {
                MediaSource::Plain(mx_uri) => {
                    let (server, id) = mx_uri.parts().unwrap();

                    let uri = format!(
                        "https://matrix-client.matrix.org/_matrix/media/v3/download/{}/{}",
                        server, id
                    );

                    let size = if let Some(file_info) = f.info.clone() {
                        match file_info.size {
                            Some(size) => {
                                let size = size.to_string();
                                let size = size.parse::<u64>();

                                if let Ok(size) = size {
                                    Some(size)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };

                    message_result = Some(TimelineMessage {
                        event_id: Some(String::from(event.as_str())),
                        sender: member.clone(),
                        body: TimelineMessageType::File(FileContent {
                            size,
                            body: f.body.clone(),
                            source: Some(ImageType::URL(uri)),
                        }),
                        origin: if member.id.eq(logged_user_id) {
                            EventOrigin::ME
                        } else {
                            EventOrigin::OTHER
                        },
                        time: timestamp,
                    });
                }
                MediaSource::Encrypted(file) => {
                    let (server, id) = file.url.parts().unwrap();

                    let uri = format!(
                        "https://matrix-client.matrix.org/_matrix/media/v3/download/{}/{}",
                        server, id
                    );

                    let size = if let Some(file_info) = f.info.clone() {
                        match file_info.size {
                            Some(size) => {
                                let size = size.to_string();
                                let size = size.parse::<u64>();

                                if let Ok(size) = size {
                                    Some(size)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };

                    message_result = Some(TimelineMessage {
                        event_id: Some(String::from(event.as_str())),
                        sender: member.clone(),
                        body: TimelineMessageType::File(FileContent {
                            size,
                            body: f.body.clone(),
                            source: Some(ImageType::URL(uri)),
                        }),
                        origin: if member.id.eq(logged_user_id) {
                            EventOrigin::ME
                        } else {
                            EventOrigin::OTHER
                        },
                        time: timestamp,
                    });
                }
            },
            MessageType::Video(video) => match &video.source {
                MediaSource::Plain(mx_uri) => {
                    let (server, id) = mx_uri.parts().unwrap();

                    let uri = format!(
                        "https://matrix-client.matrix.org/_matrix/media/v3/download/{}/{}",
                        server, id
                    );

                    let size = if let Some(file_info) = video.info.clone() {
                        match file_info.size {
                            Some(size) => {
                                let size = size.to_string();
                                let size = size.parse::<u64>();

                                if let Ok(size) = size {
                                    Some(size)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };

                    message_result = Some(TimelineMessage {
                        event_id: Some(String::from(event.as_str())),
                        sender: member.clone(),
                        body: TimelineMessageType::Video(FileContent {
                            size,
                            body: video.body.clone(),
                            source: Some(ImageType::URL(uri)),
                        }),
                        origin: if member.id.eq(logged_user_id) {
                            EventOrigin::ME
                        } else {
                            EventOrigin::OTHER
                        },
                        time: timestamp,
                    });
                }
                MediaSource::Encrypted(file) => {
                    let x = client
                        .media()
                        .get_media_content(
                            &MediaRequest {
                                source: video.source.clone(),
                                format: MediaFormat::File,
                            },
                            true,
                        )
                        .await;

                    let size = if let Some(file_info) = video.info.clone() {
                        match file_info.size {
                            Some(size) => {
                                let size = size.to_string();
                                let size = size.parse::<u64>();

                                if let Ok(size) = size {
                                    Some(size)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };

                    if let Ok(content) = x {
                        message_result = Some(TimelineMessage {
                            event_id: Some(String::from(event.as_str())),
                            sender: member.clone(),
                            body: TimelineMessageType::Video(FileContent {
                                size,
                                body: video.body.clone(),
                                source: Some(ImageType::Media(content)),
                            }),
                            origin: if member.id.eq(logged_user_id) {
                                EventOrigin::ME
                            } else {
                                EventOrigin::OTHER
                            },
                            time: timestamp,
                        });
                    }
                }
            },
            _ => {
                info!("unsuported message_type matrix");
            }
        }

        return message_result;
    }

    pub async fn format_relation_from_event(
        n: &MessageType,
        relates: &Option<Relation>,
        room: &Room,
        message_result: TimelineMessage,
        member: &RoomMember,
        logged_user_id: &str,
        time: MilliSecondsSinceUnixEpoch,
        client: &Client,
    ) -> Option<TimelineRelation> {
        match relates {
            Some(r) => match r {
                Relation::Reply { in_reply_to } => {
                    let room_event = room.event(&in_reply_to.event_id).await;
                    let timestamp = {
                        let d = UNIX_EPOCH + Duration::from_millis(time.0.into());
                        let datetime = DateTime::<Utc>::from(d);
                        datetime.format("%H:%M").to_string()
                    };

                    match room_event {
                        Ok(event) => {
                            let desc_event = event
                                .event
                                .deserialize()
                                .expect("can't deserialize event: format_relation_from_event");

                            let reply = deserialize_timeline_event(
                                desc_event,
                                room,
                                &logged_user_id,
                                &client,
                            )
                            .await;

                            match reply {
                                Some(r) => {
                                    let mut final_message = TimelineMessageReply {
                                        event: message_result,
                                        reply: Some(r.clone()),
                                    };

                                    match &r.body {
                                        TimelineMessageType::Image(_uri) => {
                                            if n.body().contains("sent an image.") {
                                                let to_remove = format!(
                                                    "> <{}> {}",
                                                    r.sender.id, "sent an image."
                                                );

                                                let uncleared_content = n.body();
                                                let n = uncleared_content
                                                    .replace(&to_remove, "")
                                                    .clone();

                                                let content_body = TimelineMessageType::Text(n);

                                                final_message.event = TimelineMessage {
                                                    event_id: None,
                                                    sender: member.clone(),
                                                    body: content_body,
                                                    origin: if member.id.eq(logged_user_id) {
                                                        EventOrigin::ME
                                                    } else {
                                                        EventOrigin::OTHER
                                                    },
                                                    time: timestamp,
                                                };
                                            }
                                        }
                                        TimelineMessageType::Text(body) => {
                                            if body.starts_with(">") {
                                                let to_remove = format!(
                                                    "> <{}> {}",
                                                    r.clone().sender.id,
                                                    body.trim()
                                                );

                                                let uncleared_content = n.body();
                                                let n = uncleared_content
                                                    .replace(&to_remove, "")
                                                    .clone();

                                                let content_body = TimelineMessageType::Text(n);

                                                final_message.event = TimelineMessage {
                                                    event_id: None,
                                                    sender: member.clone(),
                                                    body: content_body,
                                                    origin: if member.id.eq(logged_user_id) {
                                                        EventOrigin::ME
                                                    } else {
                                                        EventOrigin::OTHER
                                                    },
                                                    time: timestamp,
                                                };
                                            } else {
                                                final_message.reply = Some(r);
                                            }
                                        }
                                        TimelineMessageType::Html(_) => {
                                            final_message.reply = Some(r);
                                        }
                                        TimelineMessageType::File(_) => {
                                            final_message.reply = Some(r);
                                        }
                                        TimelineMessageType::Video(_) => {
                                            final_message.reply = Some(r);
                                        }
                                    }

                                    Some(TimelineRelation::Reply(final_message))
                                }
                                _ => None,
                            }
                        }
                        Err(_) => None,
                    }
                }
                Relation::Thread(in_reply_to) => {
                    info!(
                        "event id: {:?} \n\n {:?}",
                        in_reply_to.event_id, message_result
                    );

                    let final_message = TimelineMessageThread {
                        event_id: in_reply_to.event_id.to_string(),
                        thread: vec![message_result.clone()],
                    };

                    Some(TimelineRelation::Thread(final_message))
                }
                // Relation::Replacement(in_reply_to) => info!("replacement: {:?}", in_reply_to),
                _ => None,
            },
            None => None,
        }
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

    use matrix_sdk::ruma::api::client::account::register::v3::Request as RegistrationRequest;

    pub async fn prepare_register(
        homeserver: &str,
        username: &str,
        password: &str,
    ) -> anyhow::Result<(Client, String), Error> {
        let mut request = RegistrationRequest::new();
        request.username = Some(&username);
        request.password = Some(&password);

        let uiaa_dummy = uiaa::Dummy::new();
        request.auth = Some(uiaa::AuthData::Dummy(uiaa_dummy));

        let result = build_client(homeserver.to_string()).await;
        let (client, client_session) = match result {
            Ok((client, client_session)) => (client, client_session),
            Err(_) => panic!("Can't create client"),
        };

        match client.register(request.clone()).await {
            Ok(info) => {
                info!("{:?}", info);
                Ok((client, "registered".to_string()))
            }
            Err(error) => Err(Error::Http(error)),
        }
    }
    pub async fn register(
        homeserver: &str,
        username: &str,
        password: &str,
        recaptcha_token: Option<String>,
        session: Option<String>,
    ) -> anyhow::Result<(Client, String), Error> {
        let mut request = RegistrationRequest::new();
        request.username = Some(&username);
        request.password = Some(&password);

        if let Some(token) = &recaptcha_token {
            let mut uiaa_recaptcha = uiaa::ReCaptcha::new(&token);
            uiaa_recaptcha.session = session.as_deref();
            request.auth = Some(uiaa::AuthData::ReCaptcha(uiaa_recaptcha));
        }

        let result = build_client(homeserver.to_string()).await;
        let (client, client_session) = match result {
            Ok((client, client_session)) => (client, client_session),
            Err(_) => panic!("Can't create client"),
        };

        match client.register(request.clone()).await {
            Ok(info) => {
                info!("signup result {:?}", info);

                client.logout();

                Ok((client, "registered".to_string()))
            }
            Err(error) => Err(Error::Http(error)),
        }
    }

    pub async fn login(
        homeserver: &str,
        username: &str,
        password: &str,
    ) -> anyhow::Result<(Client, String)> {
        info!("No previous session found, logging in…");

        let (client, client_session) = build_client(homeserver.to_string()).await?;

        match client
            .login_username(&username, &password)
            .initial_device_display_name("Fido")
            .send()
            .await
        {
            Ok(info) => {
                info!("Logged in as {username}");

                info!("{:?}", info.user_id);
            }
            Err(error) => {
                info!("Error logging in: {error}");
                match error {
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

        client.restore_login(user_session.clone()).await?;

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
