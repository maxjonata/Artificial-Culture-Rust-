use bincode;
use sled::{Db, IVec};

pub struct Storage {
    db: Db,
    event_log: Db, // separa o log de eventos
}

impl Storage {
    pub fn new(path: &str) -> Self {
        let db = sled::open(format!("{}/state", path)).expect("failed to open sled");
        let event_log = sled::open(format!("{}/events", path)).expect("failed to open event log");
        Self { db, event_log }
    }

    pub fn save_component<T: Serialize>(&self, entity: EntityId, component: &T) {
        let key = entity.to_string();
        let data = bincode::serialize(component).expect("failed to serialize");
        self.db
            .insert(key.as_bytes(), data)
            .expect("failed to insert");
    }

    pub fn load_component<T: for<'de> Deserialize<'de>>(&self, entity: EntityId) -> Option<T> {
        let key = entity.to_string();
        self.db
            .get(key.as_bytes())
            .ok()
            .flatten()
            .map(|ivec| bincode::deserialize(&ivec).expect("failed to deserialize"))
    }

    pub fn append_event(&self, event: &Event) {
        let timestamp = chrono::Utc::now().timestamp_nanos().to_string();
        let data = bincode::serialize(event).expect("failed to serialize event");
        self.event_log
            .insert(timestamp.as_bytes(), data)
            .expect("failed to append event");
    }
}
