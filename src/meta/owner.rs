use ansi_term::ANSIString;
use color::{Colors, Elem};
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug)]
pub struct Owner {
    user: String,
    group: String,
}

impl<'a> From<&'a Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        let user = get_user_by_uid(meta.uid())
            .expect("failed to get user name")
            .name()
            .to_str()
            .expect("failed to convert user name to str")
            .to_string();

        let group = get_group_by_gid(meta.gid())
            .expect("failed to get the group name")
            .name()
            .to_str()
            .expect("failed to convert group name to str")
            .to_string();

        Owner { user, group }
    }
}

impl Owner {
    pub fn user(&self) -> String {
        self.user.clone()
    }

    pub fn group(&self) -> String {
        self.group.clone()
    }

    pub fn render_user(&self, user_alignment: usize) -> ANSIString {
        let mut alignment = String::with_capacity(user_alignment - self.user.len());

        for _ in 0..(user_alignment - self.user.len()) {
            alignment.push(' ');
        }

        Colors[&Elem::User].paint(alignment + &self.user)
    }

    pub fn render_group(&self, group_alignment: usize) -> ANSIString {
        let mut alignment = String::with_capacity(group_alignment - self.group.len());

        for _ in 0..(group_alignment - self.group.len()) {
            alignment.push(' ');
        }

        Colors[&Elem::Group].paint(alignment + &self.group)
    }
}
