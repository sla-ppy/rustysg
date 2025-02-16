use chrono::{NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub time: NaiveTime,
}

impl Metadata {
    /// Create a new Metadata instance from a YAML string,
    /// replacing missing fields with default values from `default_metadata`.
    pub fn from_yaml(yaml: &str, default_metadata: Option<Metadata>) -> Result<Self, serde_yml::Error> {
        let mut meta: Metadata = serde_yml::from_str(yaml)?;
        if let Some(default) = default_metadata {
            if meta.title.is_empty() {
                meta.title = default.title;
            }
            if meta.description.is_empty() {
                meta.description = default.description;
            }
            if meta.author.is_empty() {
                meta.author = default.author;
            }
            if meta.date == NaiveDate::default() {
                meta.date = default.date;
            }
            if meta.time == NaiveTime::default() {
                meta.time = default.time;
            }
        }
        Ok(meta)
    }
}

impl Context {
    pub fn new(content: String, metadata: Metadata) -> Self {
        Self { content, metadata }
    }

    /// Create a new Context instance from a file on disk, using the file's metadata
    /// where possible to substitute missing fields in the metadata.
    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(filename)?;
        // for the date and time of publish/writing, we want to default to the
        // date and time of the file itself on disk. If you create a new file on
        // 1st August 2021 at 12:00, the date and time of the file will be used
        // as the date and time of publish/writing, so that the article can say
        // "published on 1st August 2021 at 12:00".
        let file_metadata = std::fs::metadata(filename)?;
        let file_created_systime = file_metadata.created().unwrap();
        let file_created_datetime = chrono::DateTime::<Utc>::from(file_created_systime);
        let metadata = Metadata::from_yaml(&content, Some(Metadata {
            date: file_created_datetime.date_naive(),
            time: file_created_datetime.time(),
            ..Default::default()
        }))?;
        Ok(Self::new(content, metadata))
    }
}

#[derive(Debug)]
pub struct Context {
    pub content: String,
    pub metadata: Metadata,
}

impl Metadata {
}

#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn test_file_metadata_context() {
        let filename = "Cargo.toml";
        let ctx = Context::from_file(filename).unwrap();
        assert_eq!("Hello, World!", ctx.metadata.title);
        assert_eq!("This is a test.", ctx.metadata.description);
        assert_eq!("Lion", ctx.metadata.author);
        assert_eq!(NaiveDate::from_ymd_opt(2021, 8, 1).unwrap(), ctx.metadata.date);
        assert_eq!(NaiveTime::from_hms_opt(12, 0, 0).unwrap(), ctx.metadata.time);
    }
}

#[cfg(test)]
mod metadata_tests {
    use super::*;

    #[test]
    fn test_metadata_from_yaml() {
        let yaml = r#"
                title: "Hello, World!"
                description: "This is a test."
                author: "Lion"
                date: 2021-08-01
                time: 12:00:00
            "#;
        let m = Metadata::from_yaml(yaml, None).unwrap();
        assert_eq!("Hello, World!", m.title);
        assert_eq!("This is a test.", m.description);
        assert_eq!("Lion", m.author);
        assert_eq!(NaiveDate::from_ymd_opt(2021, 8, 1).unwrap(), m.date);
        assert_eq!(NaiveTime::from_hms_opt(12, 0, 0).unwrap(), m.time);

        let yaml = r#"
                title: "Hello, World!"
                description: "This is a test."
        "#;
        let m = Metadata::from_yaml(yaml, None).unwrap();
        assert_eq!("Hello, World!", m.title);
        assert_eq!("This is a test.", m.description);
        assert_eq!(String::default(), m.author);
        assert_eq!(NaiveDate::default(), m.date);
        assert_eq!(NaiveTime::default(), m.time);

        let yaml = r#"
                title: "Hello, World!"
                description: "This is a test."
        "#;
        let default = Metadata {
            title: "whatever".to_string(),
            description: "whatever".to_string(),
            author: "Lion".to_string(),
            date: NaiveDate::from_ymd_opt(2021, 8, 1).unwrap(),
            time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        };
        let m = Metadata::from_yaml(yaml, Some(default)).unwrap();
        assert_eq!("Hello, World!", m.title);
        assert_eq!("This is a test.", m.description);
        assert_eq!("Lion", m.author);
        assert_eq!(NaiveDate::from_ymd_opt(2021, 8, 1).unwrap(), m.date);
        assert_eq!(NaiveTime::from_hms_opt(12, 0, 0).unwrap(), m.time);
    }
}
