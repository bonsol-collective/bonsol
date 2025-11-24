use anyhow::Context;
use bonsol_bonfire::LogEvent;
use futures::StreamExt;
use reqwest::Url;
use reqwest_eventsource::{Event, EventSource};

pub async fn logs(
    bonfire_url: Option<String>,
    image_id: Option<String>,
    execution_id: Option<String>,
) -> anyhow::Result<()> {
    let bonfire_url =
        bonfire_url.unwrap_or_else(|| "https://bonfire.bonsol.org".to_owned()) + "/logs";

    // Build URL with optional query params
    let mut url = Url::parse(&bonfire_url).context("failed to parse logs url")?;
    {
        let mut qp = url.query_pairs_mut();
        if let Some(id) = image_id.as_ref() {
            qp.append_pair("image_id", id);
        }
        if let Some(id) = execution_id.as_ref() {
            qp.append_pair("execution_id", id);
        }
    }

    // Create an EventSource connected to the logs endpoint
    // (the API below follows the example you provided; exact names may vary by crate)
    let mut es = EventSource::get(url.as_str());

    // Stream SSE events
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => {
                // Most SSE message types expose .data (and maybe .id, .event)
                // Print the data payload as before.
                let log: LogEvent = serde_json::from_str(&message.data).unwrap();
                println!("{}", log.pretty());
            }
            Err(err) => {
                println!("Error: {}", err);
                es.close();
                break;
            }
        }
    }
    Ok(())
}
