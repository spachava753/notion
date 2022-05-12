use serde::{Deserialize, Serialize};
use crate::models::users::User;
use crate::{Database, Page};
use crate::models::properties::DateValue;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TextColor {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}

/// Rich text annotations
/// See <https://developers.notion.com/reference/rich-text#annotations>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Annotations {
    pub bold: Option<bool>,
    pub code: Option<bool>,
    pub color: Option<TextColor>,
    pub italic: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<bool>,
}

/// Properties common on all rich text objects
/// See <https://developers.notion.com/reference/rich-text#all-rich-text>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RichTextCommon {
    pub plain_text: String,
    pub href: Option<String>,
    pub annotations: Option<Annotations>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Link {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Text {
    pub content: String,
    pub link: Option<Link>,
}

/// See https://developers.notion.com/reference/rich-text#mention-objects
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum MentionObject {
    User {
        user: User
    },
    // TODO: need to add tests
    Page {
        page: Page
    },
    // TODO: need to add tests
    Database {
        database: Database
    },
    Date {
        date: DateValue
    },
    // TODO: need to add LinkPreview
    // LinkPreview {
    //
    // },
    #[serde(other)]
    Unknown
}

/// Rich text objects contain data for displaying formatted text, mentions, and equations.
/// A rich text object also contains annotations for style information.
/// Arrays of rich text objects are used within property objects and property
/// value objects to create what a user sees as a single text value in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichText {
    /// See <https://developers.notion.com/reference/rich-text#text-objects>
    Text {
        #[serde(flatten)]
        rich_text: RichTextCommon,
        text: Text,
    },
    /// See <https://developers.notion.com/reference/rich-text#mention-objects>
    Mention {
        #[serde(flatten)]
        rich_text: RichTextCommon,
        mention: MentionObject
    },
    /// See <https://developers.notion.com/reference/rich-text#equation-objects>
    Equation {
        #[serde(flatten)]
        rich_text: RichTextCommon,
    },
}

impl RichText {
    pub fn plain_text(&self) -> &str {
        use RichText::*;
        match self {
            Text { rich_text, .. } | Mention { rich_text, .. } | Equation { rich_text, .. } => {
                &rich_text.plain_text
            }
        }
    }
}
