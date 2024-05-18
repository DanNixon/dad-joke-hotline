use crate::jambonz::{Say, SaySynthesizer, Verb};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum VoiceActor {
    AwsAmy,
    AwsBrian,
    AwsEmma,
}

impl Distribution<VoiceActor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VoiceActor {
        match rng.gen_range(0..2) {
            0 => VoiceActor::AwsAmy,
            1 => VoiceActor::AwsBrian,
            2 => VoiceActor::AwsEmma,
            _ => unreachable!("Random value out of range, this should not happen"),
        }
    }
}

impl VoiceActor {
    pub(crate) fn instance(&self) -> Tts {
        let synth = match self {
            Self::AwsAmy => SaySynthesizer {
                vendor: "aws".to_string(),
                language: "en-GB".to_string(),
                gender: None,
                voice: "Amy".to_string(),
            },
            Self::AwsBrian => SaySynthesizer {
                vendor: "aws".to_string(),
                language: "en-GB".to_string(),
                gender: None,
                voice: "Brian".to_string(),
            },
            Self::AwsEmma => SaySynthesizer {
                vendor: "aws".to_string(),
                language: "en-GB".to_string(),
                gender: None,
                voice: "Emma".to_string(),
            },
        };

        Tts { synth: Some(synth) }
    }
}

pub(crate) struct Tts {
    synth: Option<SaySynthesizer>,
}

impl Tts {
    pub(crate) fn say(&self, text: &str) -> Verb {
        Verb::Say(Say {
            text: text.to_string(),
            synthesizer: self.synth.clone(),
        })
    }

    pub(crate) fn speak(&self, text: &str) -> Vec<Verb> {
        // TODO: split on punctuation and add dramatic pauses to the delivery of the joke
        vec![self.say(text)]
    }
}
