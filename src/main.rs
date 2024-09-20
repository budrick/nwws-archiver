use chrono::{DateTime, Utc};
use dotenv::dotenv;
use futures::StreamExt;
use nwws_oi::Channel;
use nwws_oi::Config;
use nwws_oi::Server;
use nwws_oi::StreamEvent;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let username = std::env::var("NWWS_OI_USERNAME").expect("NWWS_OI_USERNAME must be set");
    let password = std::env::var("NWWS_OI_PASSWORD").expect("NWWS_OI_PASSWORD must be set");

    let conf = Config {
        username,
        password,
        server: Server::Primary,
        resource: format!("uuid/{}", uuid::Uuid::new_v4()),
        channel: Channel::Default,
    };
    let mut stream = nwws_oi::Stream::new(conf);

    while let Some(event) = stream.next().await {
        match event {
            StreamEvent::ConnectionState(_state) => {}
            StreamEvent::Error(error) => tracing::error!("error: {}", error),
            StreamEvent::Message(message) => {
                let ttaa = &message.ttaaii[..4];
                let now: DateTime<Utc> = Utc::now();

                println!("{:?}", message);

                continue;

                match ttaa {
                    // Tornado warning
                    "WFUS" => {
                        println!(
                            "{}: Tornado warning issued by {}",
                            now.to_rfc3339(),
                            message.cccc
                        );
                        // let _res = stmt.execute(named_params!{":time": now.to_rfc3339(), ":type": ttaa, ":text": message.message });
                    }
                    // Severe thunderstorm warning
                    "WUUS" => {
                        // Ignore SPC updates. Unsure why they have the same WMO ttaa as severe thunderstorm warnings.
                        if message.cccc != "KWNS" {
                            println!(
                                "{}: Severe thunderstorm warning issued by {}",
                                now.to_rfc3339(),
                                message.cccc
                            );
                            // let _res = stmt.execute(named_params!{":time": now.to_rfc3339(), ":type": ttaa, ":text": message.message });
                        }
                    }
                    // Severe weather statement - may update/supercede WFUS or WUUS bulletins
                    "WWUS" => {
                        // let _res = stmt.execute(named_params!{":time": now.to_rfc3339(), ":type": ttaa, ":text": message.message });
                    }
                    // Fall through
                    _ => (),
                }
                // log::info!("{}", format!("{:#?}", message));
            }
        }
    }
}
