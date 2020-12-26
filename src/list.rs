use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum LinkType {
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "github")]
    GitHub,
}

#[derive(Debug, Serialize)]
pub struct Link {
    #[serde(rename = "type")]
    ty: LinkType,
    url: String,
}

#[derive(Debug, Serialize)]
pub struct Member {
    name: String,
    avatar: String,
    role: String,
    links: Vec<Link>,
}

impl From<crate::store::Member> for Member {
    fn from(member: crate::store::Member) -> Self {
        todo!()
    }
}

#[derive(Debug, Serialize)]
pub struct List(Vec<Member>);

impl From<crate::store::Database> for List {
    fn from(db: crate::store::Database) -> Self {
        Self(
            db.members
                .iter()
                .cloned()
                .map(|member| member.into())
                .collect(),
        )
    }
}
