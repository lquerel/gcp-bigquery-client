use gcp_bigquery_client::{
    env_vars,
    storage::{ColumnMode, ColumnType, FieldDescriptor, StreamName, TableDescriptor},
};
use prost::Message;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ref project_id, ref dataset_id, ref table_id, ref gcp_sa_key) = env_vars();

    let mut client = gcp_bigquery_client::Client::from_service_account_key_file(gcp_sa_key).await?;

    let field_descriptors = vec![
        FieldDescriptor {
            name: "actor_id".to_string(),
            number: 1,
            typ: ColumnType::Int64,
            mode: ColumnMode::Required,
        },
        FieldDescriptor {
            name: "first_name".to_string(),
            number: 2,
            typ: ColumnType::String,
            mode: ColumnMode::Required,
        },
        FieldDescriptor {
            name: "last_name".to_string(),
            number: 3,
            typ: ColumnType::String,
            mode: ColumnMode::Required,
        },
        FieldDescriptor {
            name: "last_update".to_string(),
            number: 4,
            typ: ColumnType::String,
            mode: ColumnMode::Required,
        },
    ];
    let table_descriptor = TableDescriptor { field_descriptors };

    #[derive(Clone, PartialEq, Message)]
    struct Actor {
        #[prost(int32, tag = "1")]
        actor_id: i32,

        #[prost(string, tag = "2")]
        first_name: String,

        #[prost(string, tag = "3")]
        last_name: String,

        #[prost(string, tag = "4")]
        last_update: String,
    }

    let actor1 = Actor {
        actor_id: 1,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        last_update: "2007-02-15 09:34:33 UTC".to_string(),
    };

    let actor2 = Actor {
        actor_id: 2,
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        last_update: "2008-02-15 09:34:33 UTC".to_string(),
    };

    let stream_name = StreamName::new_default(project_id.clone(), dataset_id.clone(), table_id.clone());
    let trace_id = "test_client".to_string();

    let mut streaming = client
        .storage_mut()
        .append_rows(&stream_name, &table_descriptor, &[actor1, actor2], trace_id)
        .await?;

    while let Some(resp) = streaming.next().await {
        let resp = resp?;
        println!("response: {resp:#?}");
    }

    Ok(())
}
