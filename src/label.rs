use gitlab::{
    api::{common::NameOrId, projects::labels::CreateLabel},
    types::{Label as CrateLabelStruct, LabelColor},
};
use serde::{Deserialize, Serialize};

/// An label on a project.
///
/// This is just to be deserialized form config file and should be changed to
/// `gitlab::types::Label`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    /// The name of the label.
    pub name: String,
    /// The color of the label.
    pub color: LabelColor,
    /// The description of the label.
    pub description: String,
    /// Whether or not the account connecting has subscribed to the label.
    #[serde(default)]
    pub subscribed: bool,
}

impl Label {
    pub fn to_create_label<'a, VALUE: Into<NameOrId<'a>>>(self, project: VALUE) -> CreateLabel<'a> {
        CreateLabel::builder()
            .project(project)
            .name(self.name)
            .color(self.color.value())
            .description(self.description)
            .build()
            .expect("Convert yo CreateLabel shouldn't fail")
    }
}

impl Into<CrateLabelStruct> for Label {
    fn into(self) -> CrateLabelStruct {
        let mut l = CrateLabelStruct::new(self.name, self.color);
        l.description = Some(self.description);
        l.subscribed = self.subscribed;
        l
    }
}
