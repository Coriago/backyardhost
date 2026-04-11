use crate::models::ContactEntry;
use dioxus::prelude::*;

pub async fn init_kv_bucket() -> Result<async_nats::jetstream::kv::Store, ServerFnError> {
    use async_nats::jetstream;
    use async_nats::jetstream::kv::Config;
    use async_nats::jetstream::stream::StorageType;

    let client = crate::server::nats::NatsManager::client();
    let js = jetstream::new(client.clone());

    let store = js
        .create_key_value(Config {
            bucket: "contacts".into(),
            storage: StorageType::File,
            history: 1,
            ..Default::default()
        })
        .await
        .map_err(ServerFnError::new)?;

    Ok(store)
}

pub async fn submit_entry(
    store: &async_nats::jetstream::kv::Store,
    name: String,
    email: String,
    message: String,
) -> Result<ContactEntry, ServerFnError> {
    use async_nats::jetstream::kv::UpdateErrorKind;
    use bytes::Bytes;

    let next_id = loop {
        let counter_entry = store.entry("_counter").await.map_err(ServerFnError::new)?;

        let (current, revision) = if let Some(entry) = counter_entry {
            let current = serde_json::from_slice::<u64>(&entry.value)?;
            (current, entry.revision)
        } else {
            let zero_bytes = serde_json::to_vec(&0_u64)?;
            store
                .put("_counter", Bytes::from(zero_bytes))
                .await
                .map_err(ServerFnError::new)?;
            continue;
        };

        let next = current + 1;
        let next_bytes = serde_json::to_vec(&next)?;

        match store.update("_counter", Bytes::from(next_bytes), revision).await {
            Ok(_) => break next,
            Err(err) if err.kind() == UpdateErrorKind::WrongLastRevision => continue,
            Err(err) => return Err(ServerFnError::new(err)),
        }
    };

    let entry = ContactEntry {
        id: next_id,
        name,
        email,
        message,
    };

    let payload = serde_json::to_vec(&entry)?;
    let key = format!("entry.{}", entry.id);

    store
        .put(&key, Bytes::from(payload))
        .await
        .map_err(ServerFnError::new)?;

    Ok(entry)
}

pub async fn list_entries(
    store: &async_nats::jetstream::kv::Store,
) -> Result<Vec<ContactEntry>, ServerFnError> {
    let counter_bytes = match store.get("_counter").await.map_err(ServerFnError::new)? {
        Some(bytes) => bytes,
        None => return Ok(Vec::new()),
    };

    let counter = serde_json::from_slice::<u64>(&counter_bytes)?;

    let mut entries = Vec::new();

    for id in 1..=counter {
        let key = format!("entry.{id}");
        let Some(raw) = store.get(&key).await.map_err(ServerFnError::new)? else {
            continue;
        };

        if let Ok(entry) = serde_json::from_slice::<ContactEntry>(&raw) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

#[post("/api/submit")]
pub async fn submit_entry_server(
    name: String,
    email: String,
    message: String,
) -> Result<ContactEntry, ServerFnError> {
    let store = init_kv_bucket().await?;
    submit_entry(&store, name, email, message).await
}

#[get("/api/entries")]
pub async fn list_entries_server() -> Result<Vec<ContactEntry>, ServerFnError> {
    let store = init_kv_bucket().await?;
    list_entries(&store).await
}
