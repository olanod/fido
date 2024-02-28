use std::time::Duration;

use matrix_sdk::{config::SyncSettings, Client};
use ruma::{
    api::client::{
        filter::{FilterDefinition, RoomEventFilter},
        sync::sync_events,
    },
    RoomId,
};

pub async fn sync_created_room(room_id: &RoomId, client: &Client) {
    // This is an arbitrary experiment
    // Sometimes synchronization may fail. Repeat 3 times to avoid long time of
    // waits for the user while the main sync is completed
    for _ in 0..3 {
        let room_id_list = vec![room_id.to_owned()];

        let mut room_event_filter = RoomEventFilter::default();
        room_event_filter.rooms = Some(&room_id_list);

        let mut filter = FilterDefinition::default();
        filter.room.timeline = room_event_filter;

        let sync_settings = SyncSettings::new()
            .filter(sync_events::v3::Filter::FilterDefinition(filter))
            .timeout(Duration::from_secs(30));

        let Err(_) = client.sync_once(sync_settings.clone()).await else {
            return;
        };
    }
}
