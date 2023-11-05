use std::convert::TryFrom;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A metric for classifying images.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Metric {
    /// safe for work drawings (including anime)
    Drawings,
    /// hentai and pornographic drawings
    Hentai,
    /// safe for work neutral images
    Neutral,
    /// pornographic images, sexual acts
    Porn,
    /// sexually explicit images, not pornography
    Sexy,
}

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Metric::Drawings => "Drawings",
            Metric::Hentai => "Hentai",
            Metric::Neutral => "Neutral",
            Metric::Porn => "Porn",
            Metric::Sexy => "Sexy",
        };

        write!(f, "{string}")
    }
}

impl TryFrom<usize> for Metric {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Metric::Drawings),
            1 => Ok(Metric::Hentai),
            2 => Ok(Metric::Neutral),
            3 => Ok(Metric::Porn),
            4 => Ok(Metric::Sexy),
            _ => Err("Invalid class"),
        }
    }
}

/// Represents a classification of an image and its confidence score.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Classification {
    pub metric: Metric,
    pub score: f32,
}

impl std::fmt::Display for Classification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:.2}%", self.metric, self.score * 100.0)
    }
}

#[cfg(test)]
mod test {
    use super::Metric;

    #[test]
    fn test_metric_maps() {
        assert_eq!(Metric::try_from(0).unwrap(), Metric::Drawings);
        assert_eq!(Metric::try_from(1).unwrap(), Metric::Hentai);
        assert_eq!(Metric::try_from(2).unwrap(), Metric::Neutral);
        assert_eq!(Metric::try_from(3).unwrap(), Metric::Porn);
        assert_eq!(Metric::try_from(4).unwrap(), Metric::Sexy);
        assert!(Metric::try_from(5).is_err());

        assert_eq!(Metric::Drawings.to_string(), "Drawings");
        assert_eq!(Metric::Hentai.to_string(), "Hentai");
        assert_eq!(Metric::Neutral.to_string(), "Neutral");
        assert_eq!(Metric::Porn.to_string(), "Porn");
        assert_eq!(Metric::Sexy.to_string(), "Sexy");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        use super::Classification;

        let classification = Classification {
            metric: Metric::Drawings,
            score: 0.5,
        };

        let json = serde_json::to_string(&classification).unwrap();
        assert_eq!(json, r#"{"metric":"Drawings","score":0.5}"#);

        let classification: Classification = serde_json::from_str(&json).unwrap();
        assert_eq!(classification.metric, Metric::Drawings);
        assert_eq!(classification.score, 0.5);
    }
}
