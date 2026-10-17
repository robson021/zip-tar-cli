use strum_macros::Display;

#[derive(Copy, Clone, Display)]
pub enum MainMenuItem {
    #[strum(serialize = "Create archive")]
    CreateArchive,
    #[strum(serialize = "Extract archive(s)")]
    ExtractArchives,
    #[strum(serialize = "Add to existing archive")]
    AddToExistingArchive,
    #[strum(serialize = "Exit program")]
    ExitProgram,
}

#[derive(Copy, Clone, Display)]
pub enum CreateArchivePasswordItem {
    #[strum(serialize = "No password")]
    NoPassword,
    #[strum(serialize = "Use password")]
    UsePassword,
    #[strum(serialize = "Back")]
    Back,
}

#[derive(Copy, Clone, Display)]
pub enum CreateArchiveFormatItem {
    #[strum(serialize = "ZIP")]
    Zip,
    #[strum(serialize = "TAR")]
    Tar,
    #[strum(serialize = "Back")]
    Back,
}

#[derive(Copy, Clone, Display)]
pub enum ExtractMenuItem {
    #[strum(serialize = "Extract archive(s) from a path")]
    ExtractFromPath,
    #[strum(serialize = "List all archives in current directory")]
    ListAllCurrentDir,
    #[strum(serialize = "Back")]
    Back,
}

pub const MENU_ITEMS: &[MainMenuItem] = &[
    MainMenuItem::CreateArchive,
    MainMenuItem::ExtractArchives,
    MainMenuItem::AddToExistingArchive,
    MainMenuItem::ExitProgram,
];

pub const CREATE_ARCHIVE_PASSWORD_ITEMS: &[CreateArchivePasswordItem] = &[
    CreateArchivePasswordItem::NoPassword,
    CreateArchivePasswordItem::UsePassword,
    CreateArchivePasswordItem::Back,
];

pub const CREATE_ARCHIVE_FORMAT_ITEMS: &[CreateArchiveFormatItem] = &[
    CreateArchiveFormatItem::Zip,
    CreateArchiveFormatItem::Tar,
    CreateArchiveFormatItem::Back,
];

pub const EXTRACT_MENU_ITEMS: &[ExtractMenuItem] = &[
    ExtractMenuItem::ExtractFromPath,
    ExtractMenuItem::ListAllCurrentDir,
    ExtractMenuItem::Back,
];
