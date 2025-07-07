//! The `ls` command lists the contents of a directory.

use core::panic;
use std::path::Path;

use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::commands::args::{ArgumentKind, BasicArgument, BasicArgumentKind};
use crate::commands::flags::{FlagDefinition, FlagDefinitionBuilder, FlagSpecification};
use crate::commands::{Argument, CommandOutput, ExecutableCommand, Flags};
use crate::errors::ShellError;
use crate::fs::inode::content::InodeType;
use crate::fs::inode::size::Size;
use crate::fs::permissions::Permission;
use crate::{FilePermissions, FileSystem, Inode, InodeContent};

#[derive(Clone, Default, Copy)]
pub struct LsCommand;

const fn entry_separator(display_mode: &ListDisplayMode) -> &str {
    match display_mode {
        ListDisplayMode::Long => "\n",
        ListDisplayMode::Short => "  ",
    }
}

impl ExecutableCommand for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinitionBuilder::new()
            .with_flag(FlagSpecification::new(
                "l",
                Some('l'),
                false,
                ArgumentKind::Flag,
            ))
            .with_flag(FlagSpecification::new(
                "a",
                Some('a'),
                false,
                ArgumentKind::Flag,
            ))
            .with_flag(FlagSpecification::new(
                "A",
                Some('A'),
                false,
                ArgumentKind::Flag,
            ))
            .into_flag_definition()
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

        let displayed_entries = if flags.flag("a").is_some() {
            DisplayedEntriesOptions::All
        } else if flags.flag("A").is_some() {
            DisplayedEntriesOptions::ShowDotFiles
        } else {
            DisplayedEntriesOptions::HideDotFiles
        };

        let mut output = String::new();

        match args {
            ListArgKind::Single(dir) => {
                match shell.current_session.find_inode(&shell.fs, Path::new(&dir)) {
                    Some(inode) => {
                        let inode = inode.lock().expect("Failed to lock inode");
                        let inode_type = inode.inode_type();
                        if inode_type == InodeType::Directory {
                            let contents = get_dir_contents(
                                &shell.fs,
                                &inode,
                                &display_mode,
                                &LongEntryFormatOptions::new(),
                                &displayed_entries,
                            )?;
                            output.push_str(&contents);
                        } else {
                            output.push_str(&inode.name);
                        }
                    }
                    None => {
                        output.push_str(&format!(
                            "ls: cannot access '{dir}': No such file or directory"
                        ));
                    }
                };
            }
            ListArgKind::Multiple(items) => {
                let entry_inodes = items.iter().fold(DirEntries::new(), |mut acc, item| {
                    match shell.current_session.find_inode(&shell.fs, Path::new(item)) {
                        Some(inode) => {
                            let inode = inode.lock().expect("Failed to lock inode");
                            let clone = inode.clone();
                            acc.add_entry(DirEntry { inode: clone });
                            drop(inode);
                        }
                        None => output.push_str(&format!(
                            "ls: cannot access '{item}': No such file or directory"
                        )),
                    };
                    acc
                });

                let mut entry_inodes_iter = entry_inodes.entries.iter().peekable();
                while let Some(inode) = entry_inodes_iter.next() {
                    let contents = get_dir_contents(
                        &shell.fs,
                        &inode.inode,
                        &display_mode,
                        &entry_inodes.options,
                        &displayed_entries,
                    )?;
                    output.push_str(&contents);
                    if entry_inodes_iter.peek().is_some() {
                        output.push_str(entry_separator(&display_mode));
                    }
                }
            }
        }

        if output.is_empty() {
            Ok(CommandOutput(None))
        } else {
            Ok(CommandOutput(Some(output)))
        }
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
    entry: &Inode,
    display_mode: &ListDisplayMode,
    entry_format: &LongEntryFormatOptions,
    displayed_entries: &DisplayedEntriesOptions,
) -> Result<String, ShellError> {
    let mut content = String::new();

    match &entry.content {
        InodeContent::File(_) => {
            content.push_str(&format_dir_entry(
                fs,
                &entry.name,
                entry,
                display_mode,
                entry_format,
            ));
        }
        InodeContent::Directory(dir) => {
            let mut has_entries = false;

            // Add . and ..
            if displayed_entries == &DisplayedEntriesOptions::All {
                content.push_str(&format_dir_entry(
                    fs,
                    ".",
                    entry,
                    display_mode,
                    entry_format,
                ));
                content.push_str(entry_separator(display_mode));
                has_entries = true;

                let parent = entry.parent.as_ref().expect("Parent inode should exist");
                let parent = parent.upgrade().expect("Parent inode should exist");
                let parent = parent.lock().expect("Failed to lock parent inode");
                content.push_str(&format_dir_entry(
                    fs,
                    "..",
                    &parent,
                    display_mode,
                    entry_format,
                ));
            }

            // Sort entries and filter out hidden entries (if needed)
            let mut entries = dir
                .children
                .iter()
                .filter(|(name, _)| {
                    !(name.starts_with('.')
                        && displayed_entries == &DisplayedEntriesOptions::HideDotFiles)
                })
                .collect::<Vec<_>>();
            entries.sort_by(|a, b| a.0.cmp(b.0));

            for (name, inode) in entries {
                let inode = inode.lock().expect("Failed to lock inode");

                if !(name.starts_with('.')
                    && displayed_entries == &DisplayedEntriesOptions::HideDotFiles)
                {
                    if has_entries {
                        content.push_str(entry_separator(display_mode));
                    } else {
                        has_entries = true;
                    }

                    content.push_str(&format_dir_entry(
                        fs,
                        &inode.name,
                        &inode,
                        display_mode,
                        entry_format,
                    ));
                }
            }
        }
        _ => todo!(),
    }

    Ok(content)
}

fn format_dir_entry(
    fs: &FileSystem,
    name: &str,
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

            format!("{dir}{permissions} {hard_links} {user} {group} {size} {date} {name}")
        }
        ListDisplayMode::Short => name.to_string(),
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

#[derive(PartialEq, Eq, Clone, Copy)]
enum DisplayedEntriesOptions {
    All,
    ShowDotFiles,
    HideDotFiles,
}
