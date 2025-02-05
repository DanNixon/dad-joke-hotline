use crate::{
    jambonz::{Pause, Verb},
    voice::VoiceActor,
    AppState,
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use metrics::counter;
use rand::seq::IndexedRandom;
use serde::Deserialize;
use tracing::{error, info};

pub(super) async fn call_status(Json(status): Json<crate::jambonz::CallStatusDetails>) {
    info!("Call status: {:?}", status);

    let call_status = format!("{:?}", status.call_status);
    counter!(crate::METRIC_CALLS_NAME, "status" => call_status, "from" => status.from).increment(1);
}

#[derive(Debug, Deserialize)]
struct Joke {
    #[allow(unused)]
    id: String,

    joke: String,

    #[allow(unused)]
    status: i32,
}

const ERROR_MESSAGE: &str =
    "Oh no, the internet is sad and has decided that you get no joke. Sorry about that.";

pub(super) async fn call_incoming(State(state): State<AppState>) -> Response {
    info!("Handling call");

    // Randomly select the voice actor
    let voice_actor: VoiceActor = rand::random();
    info!("Picked voice actor: {:?}", voice_actor);
    let tts = voice_actor.instance();

    // Obtain comedy
    let mut joke_verbs = match state
        .http_client
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .send()
        .await
    {
        Ok(response) => match response.json::<Joke>().await {
            Ok(joke) => {
                info!("Got joke: {:?}", joke);
                tts.speak(&joke.joke)
            }
            Err(e) => {
                error!("Error parsing joke: {e}");
                counter!(crate::METRIC_ERRORS_NAME).increment(1);
                vec![tts.say(ERROR_MESSAGE)]
            }
        },
        Err(e) => {
            error!("Error retrieving joke: {e}");
            counter!(crate::METRIC_ERRORS_NAME).increment(1);
            vec![tts.say(ERROR_MESSAGE)]
        }
    };

    // Pick a random greeting
    let greetings = [
        "Are you ready to laugh?",
        "Who wants a joke?",
        "Bet you haven't heard this one before.",
        "They don't call me \"funny telephone joke person\" for nothing!",
        "This one will have you in stitches.",
        "I know my way around a good comedy joke.",
        "How about this one?",
        "One top tier joke, coming right up!",
        "Yes, you can haz dad joke.",
    ];
    let greeting = greetings
        .choose(&mut rand::rng())
        .expect("this slice should not be empty so this should never be None");
    info!("Picked greeting: {greeting}");

    // Build response
    let mut verbs = vec![tts.say(greeting), Verb::Pause(Pause::new(1))];
    verbs.append(&mut joke_verbs);
    verbs.push(Verb::Pause(Pause::new(1)));

    Json(verbs).into_response()
}
