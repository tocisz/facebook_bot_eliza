use crate::config::CONFIG;
use crate::messages::MessagingType::RESPONSE;
use serde::{Deserialize, Serialize};

static ENDPOINT: &str = "https://graph.facebook.com/v8.0/me/messages?access_token=";

// {
//   "messaging_type": "<MESSAGING_TYPE>",
//   "recipient": {
//     "id": "<PSID>"
//   },
//   "message": {
//     "text": "hello, world!"
//   }
// }

#[derive(Deserialize, Serialize)]
#[allow(non_camel_case_types)]
enum MessagingType {
    RESPONSE,
    UPDATE,
    MESSAGE_TAG,
}

#[derive(Deserialize, Serialize)]
struct SendMessage {
    messaging_type: MessagingType,
    recipient: UserId,
    message: MessageDetails,
}

impl SendMessage {
    fn response(user: &str, message: String) -> SendMessage {
        SendMessage {
            messaging_type: RESPONSE,
            recipient: UserId {
                id: String::from(user),
            },
            message: MessageDetails { text: message },
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entries {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub id: String,
    pub time: u64,
    pub messaging: Vec<Message>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub sender: UserId,
    pub recipient: UserId,
    pub timestamp: u64,
    pub message: MessageDetails,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserId {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageDetails {
    pub text: String,
}

pub async fn send_response(user: &str, message: &str) {
    let response = format!("You said: {}", message);
    println!("[<{}] {}", user, &response);
    let url = format!("{}{}", ENDPOINT, &CONFIG.access_token);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .json(&SendMessage::response(user, response))
        .send()
        .await;
    match res {
        Ok(resp) => println!("Got response {:?}", resp),
        Err(err) => println!("Got error {:?}", err),
    }
}
