use std::fmt;

// TODO: DateTime
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub venue: String,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.name, self.venue)
    }
}

#[cfg(test)]
mod tests_event {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn event() -> Event {
        Event {
            name: "aRT and tEST".into(),
            venue: "earth-moon, milkyway".into(),
        }
    }

    #[rstest]
    fn test_display(event: Event) {
        assert_eq!(
            format!("{},{}", event.name, event.venue),
            format!("{}", event)
        );
    }
}
