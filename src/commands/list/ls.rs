//! The `ls` command lists the contents of a directory.

use std::path::Path;

use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::commands::args::{ArgumentKind, BasicArgument, BasicArgumentKind};
use crate::commands::flags::{FlagDefinition, FlagDefinitionBuilder};
use crate::commands::{Argument, CommandOutput, ExecutableCommand, Flags};
use crate::errors::{FileSystemError, ShellError};
use crate::fs::inode::content::InodeType;
use crate::fs::inode::size::Size;
use crate::fs::permissions::Permission;
use crate::sessions::Session;
use crate::{FilePermissions, FileSystem, Inode};

#[derive(Clone, Default, Copy)]
pub struct LsCommand;

const ENTRY_SEPARATOR: &str = "  ";

impl ExecutableCommand for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinitionBuilder::new().into_flag_definition()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::Enumeration(BasicArgumentKind::String))
    }

    fn execute(
        &self,
        flags: Flags,
        args: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let args = match args {
            Some(Argument::List(args)) => {
                let args = args
                    .into_iter()
                    .try_fold(Vec::new(), |mut acc, arg| match arg {
                        BasicArgument::String(arg) => {
                            acc.push(arg);
                            Ok(acc)
                        }
                        _ => Err(ShellError::Internal("Invalid argument type".to_string())),
                    })?;
                Ok(ListArgKind::Multiple(args))
            }
            None => Ok(ListArgKind::Single(".".to_string())),
            _ => Err(ShellError::Internal(
                "Bad arguments provided to ls".to_string(),
            )),
        }?;

        let display_mode = if flags.flag("l").is_some() {
            ListDisplayMode::Long
        } else {
            ListDisplayMode::Short
        };

        let mut output = String::new();

        match args {
            ListArgKind::Single(dir) => {
                let contents =
                    get_dir_contents(&shell.fs, &shell.current_session, &dir, &display_mode)?;
                output.push_str(&contents);
                output.push('\n');
            }
            ListArgKind::Multiple(items) => {
                for item in items {
                    let contents =
                        get_dir_contents(&shell.fs, &shell.current_session, &item, &display_mode)?;
                    output.push_str(&contents);
                    output.push('\n');
                }
            }
        }

        Ok(CommandOutput(None))
    }
}

struct DirEntry {
    inode: Inode,
}

struct DirEntries {
    entries: Vec<DirEntry>,
    options: LongEntryFormatOptions,
}

impl DirEntries {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            options: LongEntryFormatOptions::new(),
        }
    }

    fn add_entry(&mut self, entry: DirEntry) {
        self.options.update(&entry.inode);
        self.entries.push(entry);
    }
}

fn get_dir_contents(
    fs: &FileSystem,
    session: &Session,
    dir: &str,
    display_mode: &ListDisplayMode,
) -> Result<String, ShellError> {
    let mut content = String::new();

    let processed_entries = match fs.read_dir(dir) {
        Ok(entries) => {
            let mut processed_entries = DirEntries::new();
            for entry in entries {
                let inode = session.find_inode(fs, Path::new(&format!("{dir}/{entry}")));

                match inode {
                    Some(inode) => {
                        let locked_inode = inode.lock().expect("Failed to lock inode");
                        // TODO: Remove this clone
                        processed_entries.add_entry(DirEntry {
                            inode: locked_inode.clone(),
                        });
                    }
                    None => {
                        return Err(ShellError::Internal(format!(
                            "Failed to find inode for {dir}/{entry}"
                        )));
                    }
                }
            }
            processed_entries
        }
        Err(ShellError::FileSystem(FileSystemError::NotADirectory(path))) => {
            let inode = match session.find_inode(fs, Path::new(&path)) {
                Some(inode) => inode,
                None => return Err(ShellError::FileSystem(FileSystemError::EntryNotFound(path))),
            };
            let inode = inode.lock().expect("Failed to lock inode");

            let mut processed_entries = DirEntries::new();
            processed_entries.add_entry(DirEntry {
                inode: inode.clone(),
            });
            processed_entries
        }
        Err(e) => return Err(e),
    };

    // Format the entries
    for entry in processed_entries.entries {
        content.push_str(&format_dir_entry(
            fs,
            &entry.inode,
            display_mode,
            &processed_entries.options,
        ));
        content.push('\n');
    }

    Ok(content)
}

fn format_dir_entry(
    fs: &FileSystem,
    entry: &Inode,
    display_mode: &ListDisplayMode,
    options: &LongEntryFormatOptions,
) -> String {
    match display_mode {
        ListDisplayMode::Long => {
            let dir = format_is_dir(entry.inode_type() == InodeType::Directory);
            let permissions = format_permissions(&entry.metadata.permissions);

            let hard_links = entry.hard_link_count();

            let user = if let Some(user) = fs.get_user(entry.metadata.owner) {
                user.name.clone()
            } else {
                entry.metadata.owner.to_string()
            };

            let group = if let Some(group) = fs.get_group(entry.metadata.group) {
                group.name.clone()
            } else {
                entry.metadata.group.to_string()
            };

            let size = entry.size();

            let date = format_date(entry.metadata.created_at, options.has_dates_from_this_year);
            let name = entry.name.clone();

            format!("{dir}{permissions} {hard_links} {user} {group} {size} {date} {name}",)
        }
        ListDisplayMode::Short => {
            format!("{}{}", entry.name, ENTRY_SEPARATOR)
        }
    }
}

#[derive(Default)]
struct LongEntryFormatOptions {
    name_length: usize,
    hard_link_length: usize,
    user_length: usize,
    group_length: usize,
    size_length: usize,
    has_dates_from_this_year: bool,
}

impl LongEntryFormatOptions {
    fn new() -> Self {
        Self {
            name_length: 0,
            hard_link_length: 0,
            user_length: 0,
            group_length: 0,
            size_length: 0,
            has_dates_from_this_year: false,
        }
    }

    fn update(&mut self, entry: &Inode) {
        self.name_length = self.name_length.max(entry.name.len());
        self.hard_link_length = self
            .hard_link_length
            .max(entry.hard_link_count().to_string().len());
        self.user_length = self.user_length.max(entry.metadata.owner.to_string().len());
        self.group_length = self
            .group_length
            .max(entry.metadata.group.to_string().len());
        self.size_length = self.size_length.max(entry.size().to_string().len());
        if !self.has_dates_from_this_year && entry.metadata.created_at.year() == Utc::now().year() {
            self.has_dates_from_this_year = true;
        }
    }
}

fn format_date(date: DateTime<Utc>, has_dates_from_this_year: bool) -> String {
    let now = Utc::now();
    if date.year() == now.year() {
        format!(
            "{} {} {:02}:{:02}",
            format_month(date.month()),
            date.day(),
            date.hour(),
            date.minute()
        )
    } else {
        format!(
            "{} {} {}{}",
            format_month(date.month()),
            date.day(),
            if has_dates_from_this_year { " " } else { "" },
            date.year(),
        )
    }
}

fn format_month(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => panic!("Invalid month: {month}"),
    }
}

fn format_is_dir(is_dir: bool) -> char {
    if is_dir { 'd' } else { '-' }
}

fn format_permissions(permissions: &FilePermissions) -> String {
    format!(
        "{}{}{}",
        format_permission(&permissions.user()),
        format_permission(&permissions.group()),
        format_permission(&permissions.other())
    )
}

fn format_permission(permission: &Permission) -> String {
    format!(
        "{}{}{}",
        if permission.read { 'r' } else { '-' },
        if permission.write { 'w' } else { '-' },
        if permission.execute { 'x' } else { '-' }
    )
}

enum ListArgKind {
    Single(String),
    Multiple(Vec<String>),
}

enum ListDisplayMode {
    Long,
    Short,
}
