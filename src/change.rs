use crate::errors::{GumboError, Result};
use crate::templates::ensure_directory_exists;
use crate::templates::main::append_service;
use crate::templates::modrs::append_module;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Action {
    Append,
    Truncate,
    // the change will result in a service being added to the actix routes
    AppendService,
}

#[derive(Debug)]
pub(crate) enum Content {
    Text(String),
    Bytes(Vec<u8>),
    Route(String),
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Content::Text(value)
    }
}
impl From<&String> for Content {
    fn from(value: &String) -> Self {
        Content::Text(value.to_string())
    }
}
impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Content::Text(value.to_string())
    }
}
impl From<Vec<u8>> for Content {
    fn from(value: Vec<u8>) -> Self {
        Content::Bytes(value)
    }
}
impl From<&[u8]> for Content {
    fn from(value: &[u8]) -> Self {
        Content::Bytes(value.to_vec())
    }
}

pub(crate) struct Change {
    file: PathBuf,
    content: Content,
    action: Action,
    add_parent_mod: bool,
}

impl Change {
    /// Create a new file change event
    pub(crate) fn new(path: impl Into<String>, content: impl Into<Content>) -> Result<Self> {
        let path: String = path.into();
        let file = PathBuf::from_str(&path).map_err(|_| GumboError::InvalidPathStr(path))?;
        Ok(Self {
            file,
            content: content.into(),
            action: Action::Truncate,
            add_parent_mod: false,
        })
    }

    /// Create a new file change event
    pub(crate) fn new_from_path(
        path: impl Into<PathBuf>,
        content: impl Into<Content>,
    ) -> Result<Self> {
        let file: PathBuf = path.into();
        Ok(Self {
            file,
            content: content.into(),
            action: Action::Truncate,
            add_parent_mod: false,
        })
    }

    /// append_service
    pub(crate) fn append_service(service: impl Into<String>) -> Result<Self> {
        let path = "./src/main.rs";
        let file =
            PathBuf::from_str(path).map_err(|_| GumboError::InvalidPathStr(path.to_string()))?;
        Ok(Self {
            file,
            content: Content::Route(service.into()),
            action: Action::AppendService,
            add_parent_mod: false,
        })
    }

    /// make this change append to the contents of the file
    pub(crate) fn file(&self) -> &Path {
        &self.file
    }

    /// make this change append to the contents of the file
    pub(crate) fn append(mut self) -> Self {
        self.action = Action::Append;
        self
    }

    /// make this change add the file to its parent module
    pub(crate) fn add_parent_mod(mut self) -> Self {
        self.add_parent_mod = true;
        self
    }

    // return the name mod this change belongs to
    fn modname(&self) -> Option<String> {
        let name = self.file.file_stem()?;
        let name = name.to_str()?;
        if name != "mod" {
            return Some(name.to_string());
        }
        let name = self.file.parent()?.file_name()?;
        Some(name.to_str()?.to_string())
    }

    // return the parent file that this mod is defined in.
    fn parent_mod(&self) -> Option<PathBuf> {
        let path = self.file.to_path_buf().clone();
        let name = path.file_stem()?.to_str()?;
        if name == "mod" {
            let mut parent = path.parent()?.parent()?.to_path_buf();
            parent.push("mod.rs");
            return Some(parent);
        }
        let mut parent = path.parent()?.to_path_buf();
        parent.push("mod.rs");
        Some(parent)
    }
}

/// make this change add the file to its parent module
fn write_change_to_disk(rootpath: &Path, change: &Change) -> Result<()> {
    let mut fullpath = rootpath.to_path_buf();
    fullpath.push(&change.file);

    ensure_directory_exists(&fullpath)?;

    let mut file = match change.action {
        Action::Append => File::options().create(true).append(true).open(fullpath)?,
        Action::Truncate => File::create(fullpath)?,
        Action::AppendService => {
            let route = match &change.content {
                Content::Route(r) => r,
                _ => panic!(),
            };
            append_service(&fullpath, route)?;
            return Ok(());
        }
    };

    if change.add_parent_mod
        && let Some(modname) = change.modname()
        && let Some(modpath) = change.parent_mod()
    {
        append_module(rootpath, &modpath, &modname)?;
    }

    match &change.content {
        Content::Text(x) => file.write_all(x.as_bytes())?,
        Content::Bytes(x) => file.write_all(x.as_slice())?,
        _ => panic!(),
    };

    Ok(())
}

/// writes all changes to disk, then bubbles up any errors.
/// doesn't stop the write for one failed change.
pub(crate) fn write_to_disk<'c, I>(root_path: &Path, changes: I) -> Result<()>
where
    I: Iterator<Item = &'c Change>,
{
    // write all the changes, then bubble up any Error
    let mut results = Vec::default();
    for change in changes {
        results.push(write_change_to_disk(root_path, change));
    }
    let results: std::result::Result<Vec<_>, _> = results.into_iter().collect();
    results?;

    Ok(())
}
