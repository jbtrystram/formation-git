use std::str::FromStr;

pub struct Sentences {
    pub hello: String,
    pub bye: String,
    pub food: String,
    pub thanks: String,
}

pub enum Language {
    French,
    English,
    Portugese,
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fr" | "french" | "French" => Ok(Language::French),
            "en" | "english" | "English" => Ok(Language::English),
            _ => Err("no match"),
        }
    }
}

impl Sentences {
    pub fn new(input: Language) -> Self {
        match input {
            Language::English => Sentences {
                hello: "Hello ! I am a traveller ! How are you ?".to_string(),
                bye: "See you later ! Bye bye !".to_string(),
                food: "I am hungry ! Is there a good restaurant around ?".to_string(),
                thanks: "Thanks you so much for helping me !".to_string(),
            },
            Language::French => Sentences {
                hello: "Bonjour ! Je suis un voyageur, comment allez vous ?".to_string(),
                bye: "Au revoir et à bientôt !".to_string(),
                food: "Je suis affamé ! Connaissez vous de bons restaurants dans le quartier ?"
                    .to_string(),
                thanks: "Merci pour votre aide précieuse.".to_string(),
            },
        }
    }
}
