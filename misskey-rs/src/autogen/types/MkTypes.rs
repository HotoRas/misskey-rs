use std::collections::HashMap;

pub type UrlStr = String;

pub struct EError {
    pub code: String,
    pub message: String,
    pub id: String,
}

pub struct Error {
    pub error: EError,
}

pub struct AvatarDecoration {
    pub id: String,
    pub angle: Option<i16>,
    pub flipH: Option<bool>,
    pub url: UrlStr,
}

pub struct Instance {
    pub name: Option<String>,
    pub softwareName: Option<String>,
    pub softwareVersion: Option<String>,
    pub iconUrl: Option<UrlStr>,
    pub faviconUrl: Option<UrlStr>,
    pub themeColor: Option<String>,
}

pub type Emojis = HashMap<String, String>;

pub enum OnlineStatus {
    unknown,
    online,
    active,
    offline,
}

pub struct BadgeRole {
    pub name: String,
    pub iconUrl: Option<UrlStr>,
    pub displayOrder: u16,
    pub behavior: Option<String>,
}

pub struct UserLite {
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub host: Option<String>,
    pub avatarUrl: Option<UrlStr>,
    pub avatarBlurhash: Option<UrlStr>,

    pub avatarDecorations: Vec<AvatarDecoration>,
    pub isBot: Option<bool>,
    pub isCat: Option<bool>,
    pub instance: Option<Instance>,

    pub emojis: Emojis,
    pub onlineStatus: OnlineStatus,
    pub badgeRoles: Vec<BadgeRole>,
}

pub type User = UserLite;

pub struct UserFields {
    pub name: String,
    pub value: String,
}

pub enum Visibility {
    public,
    followers,
    private,
}

pub enum NotifyConf {
    normal,
    none,
}

pub struct UserDetailedNotMeOnly {
    pub url: Option<UrlStr>,
    pub uri: Option<UrlStr>,
    pub movedTo: Option<UrlStr>,
    pub alsoKnownAs: Option<Vec<String>>,

    pub createdAt: String,             // -> chrono::DateTime
    pub updatedAt: Option<String>,     // -> chrono::DateTime
    pub lastFetchedAt: Option<String>, // -> chrono::DateTime

    pub bannerUrl: Option<UrlStr>,
    pub bannerBlurhash: Option<String>,

    pub isLocked: bool,
    pub isSilenced: bool,
    pub isLimited: bool,
    pub isSuspended: bool,

    pub description: Option<String>,
    pub location: Option<String>,
    pub birthday: Option<String>, // YYYY-MM-DD
    pub lang: Option<String>,

    pub fields: Vec<UserFields>,
    pub verifiedLinks: Vec<UrlStr>,
    pub followersCount: u32,
    pub followingCount: u32,
    pub notesCount: u32,

    pub pinnedNoteIds: Vec<String>,
    pub pinnedNotes: Vec<Note>,
    pub pinnedPageId: Option<String>,
    pub pinnedPage: Option<Page>,

    pub publicReactions: bool,
    pub followingVisibility: Visibility,
    pub followersVisibility: Visibility,

    pub twoFactorEnabled: bool,
    pub usePasswordLessLogin: bool,
    pub securityKeys: bool,

    pub roles: Vec<RoleLite>,
    pub memo: Option<String>,
    pub moderationNote: Option<String>,

    pub isFollowing: Option<bool>,
    pub isFollowed: Option<bool>,
    pub hasPendingFollowRequestFromYou: Option<bool>,
    pub hasPendingFollowRequestToYou: Option<bool>,

    pub isBlocking: Option<bool>,
    pub isBlocked: Option<bool>,
    pub isMuted: Option<bool>,
    pub isRenoteMuted: Option<bool>,

    pub notify: NotifyConf,
    pub withReplies: Option<bool>,
}

pub struct UserDetailedNotMe {
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub host: Option<String>,
    pub avatarUrl: Option<UrlStr>,
    pub avatarBlurhash: Option<UrlStr>,

    pub avatarDecorations: Vec<AvatarDecoration>,
    pub isBot: Option<bool>,
    pub isCat: Option<bool>,
    pub instance: Option<Instance>,

    pub emojis: Emojis,
    pub onlineStatus: OnlineStatus,
    pub badgeRoles: Vec<BadgeRole>,

    pub url: Option<UrlStr>,
    pub uri: Option<UrlStr>,
    pub movedTo: Option<UrlStr>,
    pub alsoKnownAs: Option<Vec<String>>,

    pub createdAt: String,             // -> chrono::DateTime
    pub updatedAt: Option<String>,     // -> chrono::DateTime
    pub lastFetchedAt: Option<String>, // -> chrono::DateTime

    pub bannerUrl: Option<UrlStr>,
    pub bannerBlurhash: Option<String>,

    pub isLocked: bool,
    pub isSilenced: bool,
    pub isLimited: bool,
    pub isSuspended: bool,

    pub description: Option<String>,
    pub location: Option<String>,
    pub birthday: Option<String>, // YYYY-MM-DD
    pub lang: Option<String>,

    pub fields: Vec<UserFields>,
    pub verifiedLinks: Vec<UrlStr>,
    pub followersCount: u32,
    pub followingCount: u32,
    pub notesCount: u32,

    pub pinnedNoteIds: Vec<String>,
    pub pinnedNotes: Vec<Note>,
    pub pinnedPageId: Option<String>,
    pub pinnedPage: Option<Page>,

    pub publicReactions: bool,
    pub followingVisibility: Visibility,
    pub followersVisibility: Visibility,

    pub twoFactorEnabled: bool,
    pub usePasswordLessLogin: bool,
    pub securityKeys: bool,

    pub roles: Vec<RoleLite>,
    pub memo: Option<String>,
    pub moderationNote: Option<String>,

    pub isFollowing: Option<bool>,
    pub isFollowed: Option<bool>,
    pub hasPendingFollowRequestFromYou: Option<bool>,
    pub hasPendingFollowRequestToYou: Option<bool>,

    pub isBlocking: Option<bool>,
    pub isBlocked: Option<bool>,
    pub isMuted: Option<bool>,
    pub isRenoteMuted: Option<bool>,

    pub notify: NotifyConf,
    pub withReplies: Option<bool>,
}

pub type UserDetailed = UserDetailedNotMe;

pub enum TwoFactorBackupStock {
    full,
    partial,
    none,
}

pub enum NotifyFromOneOf {
    all,
    following,
    follower,
    muturalFollow,
    followingOrFollower,
    never,
    list(String),
}

pub struct NotificationRecieveConf {
    pub note: Option<NotifyFromOneOf>,
    pub follow: Option<NotifyFromOneOf>,
    pub mention: Option<NotifyFromOneOf>,
    pub reply: Option<NotifyFromOneOf>,
    pub renote: Option<NotifyFromOneOf>,
    pub quote: Option<NotifyFromOneOf>,
    pub reaction: Option<NotifyFromOneOf>,
    pub pollEnded: Option<NotifyFromOneOf>,
    pub receiveFollowRequest: Option<NotifyFromOneOf>,
    pub followRequestAccepted: Option<NotifyFromOneOf>,
    pub roleAssigned: Option<NotifyFromOneOf>,
    pub achivementEarned: Option<NotifyFromOneOf>,
    pub app: Option<NotifyFromOneOf>,
    pub test: Option<NotifyFromOneOf>,
}

pub type Achivements = HashMap<String, i64>; // name, timestamp

pub struct SecurityKeysList {
    pub id: String,
    pub name: String,
    pub lastUsed: String,
}

pub struct MeDetailedOnly {
    pub avatarId: Option<String>,
    pub bannerId: Option<String>,
    pub isModerator: Option<bool>,
    pub isAdmin: Option<bool>,

    pub injectFeaturedNote: bool,
    pub recieveAnnouncementEmail: bool,
    pub alwaysMarkNsfw: bool,
    pub autoSensitive: bool,

    pub carefulBot: bool,
    pub autoAcceptFollowed: bool,
    pub noCrawle: bool,
    pub preventAiLearning: bool,
    pub isExplorable: bool,
    pub isDeleted: bool,

    pub twoFactorBackupCodesStock: TwoFactorBackupStock,
    pub hideOnlineStatus: bool,
    pub hasUnreadSpecifiedNotes: bool,
    pub hasUnreadMentions: bool,
    pub unreadAnnouncements: Vec<Announcement>,
    pub hasUnreadAntenna: bool,
    pub hasUnreadChannel: bool,
    pub hasUnreadNotification: bool,
    pub hadPendingRecievedFollowRequest: bool,
    pub unreadNotificationsCount: u16,
    pub mutedWords: Vec<Vec<String>>,
    pub mutedInstances: Option<Vec<UrlStr>>,

    pub notificationRecieveConfig: NotificationRecieveConf,
    pub emailNotificationTypes: Vec<String>,
    pub achivements: Achivements,
    pub loggedInDays: u32,
    pub policies: RolePolicies,

    pub email: Option<String>, // email
    pub emailVerified: Option<bool>,
    pub securityKeysList: Option<Vec<SecurityKeysList>>,
    pub token: Option<String>,
}

pub struct MeDetailed {
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub host: Option<String>,
    pub avatarUrl: Option<UrlStr>,
    pub avatarBlurhash: Option<UrlStr>,

    pub avatarDecorations: Vec<AvatarDecoration>,
    pub isBot: Option<bool>,
    pub isCat: Option<bool>,
    pub instance: Option<Instance>,

    pub emojis: Emojis,
    pub onlineStatus: OnlineStatus,
    pub badgeRoles: Vec<BadgeRole>,

    pub url: Option<UrlStr>,
    pub uri: Option<UrlStr>,
    pub movedTo: Option<UrlStr>,
    pub alsoKnownAs: Option<Vec<String>>,

    pub createdAt: String,             // -> chrono::DateTime
    pub updatedAt: Option<String>,     // -> chrono::DateTime
    pub lastFetchedAt: Option<String>, // -> chrono::DateTime

    pub bannerUrl: Option<UrlStr>,
    pub bannerBlurhash: Option<String>,

    pub isLocked: bool,
    pub isSilenced: bool,
    pub isLimited: bool,
    pub isSuspended: bool,

    pub description: Option<String>,
    pub location: Option<String>,
    pub birthday: Option<String>, // YYYY-MM-DD
    pub lang: Option<String>,

    pub fields: Vec<UserFields>,
    pub verifiedLinks: Vec<UrlStr>,
    pub followersCount: u32,
    pub followingCount: u32,
    pub notesCount: u32,

    pub pinnedNoteIds: Vec<String>,
    pub pinnedNotes: Vec<Note>,
    pub pinnedPageId: Option<String>,
    pub pinnedPage: Option<Page>,

    pub publicReactions: bool,
    pub followingVisibility: Visibility,
    pub followersVisibility: Visibility,

    pub twoFactorEnabled: bool,
    pub usePasswordLessLogin: bool,
    pub securityKeys: bool,

    pub roles: Vec<RoleLite>,
    pub memo: Option<String>,
    pub moderationNote: Option<String>,

    pub isFollowing: Option<bool>,
    pub isFollowed: Option<bool>,
    pub hasPendingFollowRequestFromYou: Option<bool>,
    pub hasPendingFollowRequestToYou: Option<bool>,

    pub isBlocking: Option<bool>,
    pub isBlocked: Option<bool>,
    pub isMuted: Option<bool>,
    pub isRenoteMuted: Option<bool>,

    pub notify: NotifyConf,
    pub withReplies: Option<bool>,

    pub avatarId: Option<String>,
    pub bannerId: Option<String>,
    pub isModerator: Option<bool>,
    pub isAdmin: Option<bool>,

    pub injectFeaturedNote: bool,
    pub recieveAnnouncementEmail: bool,
    pub alwaysMarkNsfw: bool,
    pub autoSensitive: bool,

    pub carefulBot: bool,
    pub autoAcceptFollowed: bool,
    pub noCrawle: bool,
    pub preventAiLearning: bool,
    pub isExplorable: bool,
    pub isDeleted: bool,

    pub twoFactorBackupCodesStock: TwoFactorBackupStock,
    pub hideOnlineStatus: bool,
    pub hasUnreadSpecifiedNotes: bool,
    pub hasUnreadMentions: bool,
    pub unreadAnnouncements: Vec<Announcement>,
    pub hasUnreadAntenna: bool,
    pub hasUnreadChannel: bool,
    pub hasUnreadNotification: bool,
    pub hadPendingRecievedFollowRequest: bool,
    pub unreadNotificationsCount: u16,
    pub mutedWords: Vec<Vec<String>>,
    pub mutedInstances: Option<Vec<UrlStr>>,

    pub notificationRecieveConfig: NotificationRecieveConf,
    pub emailNotificationTypes: Vec<String>,
    pub achivements: Achivements,
    pub loggedInDays: u32,
    pub policies: RolePolicies,

    pub email: Option<String>, // email
    pub emailVerified: Option<bool>,
    pub securityKeysList: Option<Vec<SecurityKeysList>>,
    pub token: Option<String>,
}

pub struct UserList {
    pub id: String,
    pub createdAt: String, // datetime
    pub name: String,
    pub userIds: Option<String>,
    pub isPublic: bool,
}

pub struct Ad {
    pub id: String,
    pub expiresAt: String, // datetime
    pub startsAt: String,  // datetime
    pub place: String,
    pub priority: String,
    pub ratio: u16,
    pub url: UrlStr,
    pub imageUrl: UrlStr,
    pub memo: String,
    pub dayOfWeek: u8,
}

pub enum AnnouncementIcon {
    info,
    warning,
    error,
    success,
}

pub enum AnnouncementDisplay {
    dialog,
    normal,
    banner,
}

pub struct Announcement {
    pub id: String,
    pub createdAt: String,
    pub updatedAt: Option<String>,
    pub text: String,
    pub title: String,
    pub imageUrl: Option<UrlStr>,
    pub icon: AnnouncementIcon,
    pub display: AnnouncementDisplay,
    pub needConfirmationToRead: bool,
    pub silence: bool,
    pub forYou: bool,
    pub isRead: Option<bool>,
}

pub struct App {
    pub id: String,
    pub name: String,
    pub callbackUrl: Option<UrlStr>,
    pub permission: Vec<String>,
    pub secret: Option<String>,
    pub isAuthorized: Option<bool>,
}

pub enum NoteVisibility {
    public,
    home,
    followers,
    specified,
}

pub struct NotePollChoice {
    pub isVoted: bool,
    pub text: String,
    pub votes: u32,
}

pub struct NotePoll {
    pub expiresAt: Option<String>, // datetime
    pub multiple: bool,
    pub choices: Vec<NotePollChoice>,
}

pub struct NoteChannel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub isSensitive: bool,
    pub allowRenoteToExternal: bool,
    pub userId: Option<String>,
}

pub struct Note {
    pub id: String,
    pub createdAt: String, // datetime
    // pub editedAt: Option<String>, // datetime
    // pub editedAtHistory: Option<Vec<String>>, // datetime[]
    // pub editHistory: Option<Vec<String>>,
    pub deletedAt: Option<String>,

    pub text: Option<String>,
    pub cw: Option<String>,

    pub userId: String,
    pub user: UserLite,
    pub replyId: Option<String>,
    pub renoteId: Option<String>,
    pub reply: Option<Box<Note>>,
    pub renote: Option<Box<Note>>,

    pub isHidden: bool,
    pub visibility: NoteVisibility,

    pub mentions: Option<Vec<String>>,
    pub visibleUserIds: Option<Vec<String>>,

    pub fileIds: Option<Vec<String>>,
    pub files: Option<Vec<DriveFile>>,

    pub tags: Vec<String>,
    pub poll: Option<NotePoll>,

    pub emojis: Vec<HashMap<String, String>>,

    pub channelId: Option<String>,
    pub channel: Option<NoteChannel>,

    pub localOnly: Option<bool>,
    pub reactionAcceptance: Option<String>,
    pub reactionEmojis: Vec<HashMap<String, String>>,
    pub reactions: Vec<HashMap<String, u32>>,
    pub reactionCount: u32,
    pub renoteCount: u32,
    pub repliesCount: u32,

    pub uri: Option<UrlStr>,
    pub url: Option<UrlStr>,

    pub reactionAndUserPairCache: Option<Vec<String>>,
    pub clippedCount: Option<u32>,
    pub myReaction: Option<String>,
}

pub struct NoteReaction {
    pub id: String,
    pub createdAt: String,
    pub user: UserLite,
    pub r#type: String,
}

pub struct NoteFavorite {
    pub id: String,
    pub createdAt: String,
    pub note: Note,
    pub noteId: String,
}

pub enum Notification {
    note {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
        note: Note,
    },
    mention {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
        note: Note,
    },
    renote {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
        note: Note,
    },
    quote {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
        note: Note,
    },
    reaction {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
        note: Note,
        reaction: String,
    },
    pollEnded {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
        note: Note,
    },
    follow {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
    },
    followRequestAccepted {
        id: String,
        createdAt: String,
        r#type: String,
        user: UserLite,
        userId: String,
    },
    roleAssigned {
        id: String,
        createdAt: String,
        r#type: String,
        role: Role,
    },
    achivementEarned {
        id: String,
        createdAt: String,
        r#type: String,
        achivement: String,
    },
    app {
        id: String,
        createdAt: String,
        r#type: String,
        body: String,
        header: String,
        icon: String,
    },
    reaction_Grouped {
        id: String,
        createdAt: String,
        r#type: String,
        note: Note,
        reactions: Vec<NotifyReaction>,
    },
    renote_Grouped {
        id: String,
        createdAt: String,
        r#type: String,
        note: Note,
        users: Vec<UserLite>,
    },
    test {
        id: String,
        createdAt: String,
        r#type: String,
    },
}

pub struct NotifyReaction {
    pub user: UserLite,
    pub reaction: String,
}

pub struct DriveFileProps {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub orientation: Option<u8>,
    pub avgColor: Option<String>,
}

pub struct DriveFile {
    pub id: String,
    pub createdAt: String,
    pub name: String,
    pub r#type: String,
    pub md5: String,
    pub size: u32,
    pub isSensitive: bool,
    pub blurhash: Option<String>,
    pub properties: DriveFileProps,
    pub url: UrlStr,
    pub thumbnailUrl: Option<UrlStr>,
    pub comment: Option<String>,
    pub folderId: Option<String>,
    pub folder: Option<Box<DriveFolder>>,
    pub userId: Option<String>,
    pub user: Option<Box<UserLite>>,
}

pub struct DriveFolder {
    pub id: String,
    pub createdAt: String, // datetime
    pub name: String,
    pub parentId: Option<String>,
    pub foldersCount: Option<u16>,
    pub filesCount: Option<u16>,
    pub parent: Option<Box<DriveFolder>>,
}

pub struct Following {
    pub id: String,
    pub createdAt: String, // datetime
    pub followeeId: String,
    pub followerId: String,
    pub followee: Option<UserDetailedNotMe>,
    pub follower: Option<UserDetailedNotMe>,
}

pub struct Muting {
    pub id: String,
    pub createdAt: String,
    pub expiresAt: String,
    pub muteeId: String,
    pub mutee: UserDetailedNotMe,
}

pub struct RenoteMuting {
    pub id: String,
    pub createdAt: String,
    pub muteeId: String,
    pub mutee: UserDetailedNotMe,
}

pub struct blocking {
    pub id: String,
    pub createdAt: String,
    pub blockeeId: String,
    pub blockee: UserDetailedNotMe,
}

pub struct Hashtag {
    pub tag: String,
    pub mentionedUsersCount: u32,
    pub mentionedLocalUsersCount: u32,
    pub mentionedRemoteUsersCount: u32,
    pub attachedUsersCount: u32,
    pub attachedLocalUsersCount: u32,
    pub attachedRemoteUsersCount: u32,
}

pub struct InviteCode {
    pub id: String,
    pub code: String,
    pub expiresAt: Option<String>, // @nullable datetime
    pub createdAt: String,         // datetime
    pub createdBy: Option<UserLite>,
    pub usedBy: Option<UserLite>,
    pub UsedAt: Option<String>, // datetime
    pub used: bool,
}

pub struct Page {
    pub id: String,
    pub createdAt: String, // datetime
    pub updatedAt: String, // datetime
    pub userId: String,
    pub user: UserLite,
    pub content: Vec<PageBlock>,
    pub variables: Vec<HashMap<String, !>>,

    pub title: String,
    pub name: String,
    pub summary: Option<String>,
    pub hideTitleWhenPinned: bool,

    pub alignCenter: bool,
    pub font: String,
    pub script: String,
    pub eyeCatchingImageId: Option<String>,
    pub eyeCatchingImage: Option<DriveFile>,
    pub attachedFile: Vec<DriveFile>,

    pub likedCount: u16,
    pub isLiked: Option<bool>,
}

pub enum PageBlock {
    text {
        id: String,
        r#type: String,
        text: String,
    },
    section {
        id: String,
        r#type: String,
        title: String,
        children: Vec<PageBlock>,
    },
    image {
        id: String,
        r#type: String,
        fileId: Option<String>,
    },
    note {
        id: String,
        r#type: String,
        detailed: bool,
        note: Option<String>,
    },
}

pub struct Channel {
    pub id: String,
    pub createdAt: String,           // datetime
    pub lastNotedAt: Option<String>, // @nullable datetime
    pub name: String,
    pub description: Option<String>,
    pub bannerUrl: Option<UrlStr>,
    pub pinnedNoteIds: Vec<String>,
    pub color: String,
    pub isArchived: bool,
    pub usersCount: u16,
    pub notesCount: u32,
    pub isSensitive: bool,
    pub allowRenoteToExternal: bool,
    pub isFollowing: Option<bool>,
    pub isFavorited: Option<bool>,
    pub pinnedNotes: Option<Vec<Note>>,
}

pub struct QueueCount {
    // this is "copy"
    pub waiting: u32,
    pub active: u32,
    pub completed: u32,
    pub failed: u32,
    pub delayed: u32,
}

pub struct Antenna {
    pub id: String,
    pub createdAt: String, // datetime
    pub name: String,
    pub keywords: Vec<Vec<String>>,
    pub excludeKeywords: Vec<Vec<String>>,
    pub src: AntennaSrc,
    pub userListId: Option<String>,
    pub users: Vec<String>,
    pub caseSensitive: bool,
    pub localOnly: bool,
    pub excludeBots: bool,
    pub withReplies: bool,
    pub withFile: bool,
    pub isActive: bool,
    pub hasUnreadNote: bool,
    pub notify: bool,
}

pub enum AntennaSrc {
    home,
    all,
    users,
    list,
    users_blacklist,
}

pub struct Clip {
    pub id: String,
    pub createdAt: String,             // datetime
    pub lastClippedAt: Option<String>, // @nullable datetime
    pub userId: String,
    pub user: UserLite,
    pub name: String,
    pub description: Option<String>,
    pub isPublic: bool,
    pub favoritedCount: u16,
    pub isFavorited: Option<bool>,
    pub notesCount: Option<u16>,
}

pub enum FederationSuspensionState {
    none,
    manuallySuspended,
    goneSuspended,
    autoSuspendedForNotResponding,
}

pub struct FederationInstance {
    pub id: String,
    pub firstRecievedAt: String, // datetime
    pub host: UrlStr,
    pub usersCount: u16,
    pub notesCount: u32,
    pub followingCount: u16,
    pub followersCount: u16,

    pub isNotResponding: bool,
    pub isSuspended: bool,
    pub suspensionState: FederationSuspensionState,

    pub isBlocked: bool,
    pub softwareName: Option<String>,
    pub softwareVersion: Option<String>,
    pub openRegistration: Option<bool>,

    pub name: Option<String>,
    pub description: Option<String>,
    pub maintainerName: Option<String>,
    pub maintainerEmail: Option<String>,
    pub isSilenced: bool,
    pub iconUrl: Option<UrlStr>,
    pub faviconUrl: Option<UrlStr>,
    pub themeColor: Option<String>,
    pub infoUpdatedAt: Option<String>, // @nullable datetime
    pub latestRequestRecievedAt: Option<String>, // @nullable datetime
    pub moderationNote: Option<String>,
}

pub struct GalleryPost {
    pub id: String,
    pub createdAt: String, // datetime
    pub updatedAt: String, // datetime
    pub userId: String,
    pub user: UserLite,
    pub title: String,
    pub description: Option<String>,
    pub fileIds: Option<Vec<String>>,
    pub files: Option<Vec<DriveFile>>,
    pub tags: Option<Vec<String>>,
    pub isSensitive: bool,
    pub likedCount: u16,
    pub isLiked: Option<bool>,
}

pub struct EmojiSimple {
    pub aliases: Vec<String>,
    pub name: String,
    pub category: Option<String>,
    pub url: UrlStr,
    pub localOnly: Option<bool>,
    pub isSensitive: Option<bool>,
    pub roleIdsThatCanBeUsedThisEmojiAsReaction: Option<Vec<String>>,
}

pub struct EmojiDetailed {
    pub id: String,
    pub aliases: Vec<String>,
    pub name: String,
    pub category: Option<String>,

    pub host: Option<String>,
    pub url: UrlStr,
    pub license: Option<String>,

    pub isSensitive: bool,
    pub localOnly: bool,
    pub roleIdsThatCanBeUsedThisEmojiAsReaction: Vec<String>,
}

pub struct Flash {
    pub id: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub userId: String,
    pub user: UserLite,
    pub title: String,
    pub summary: String,
    pub script: String,
    pub likedCount: Option<u16>,
    pub isLiked: Option<bool>,
}

pub struct Signin {
    pub id: String,
    pub createdAt: String,
    pub ip: String,
    pub headers: HashMap<String, Box<dyn Fn() -> ()>>,
    pub success: bool,
}

pub enum RoleCondTypes {
    and(RoleCondFormulaValue),
    or(RoleCondFormulaValue),
    not(RoleCondFormulaValue),
    isLocal,
    isRemote,
    isSuspended,
    isLocked,
    isBot,
    isCat,
    isExplorable,
    roleAssignedTo(String),
    createdLessThan(u64),
    createdMoreThan(u64),
    followersLessThanOrEq(u32),
    followersMoreThanOrEq(u32),
    followingLessThanOrEq(u32),
    followingMoreThanOrEq(u32),
    notesLessThanOrEq(u32),
    notesMoreThanOrEq(u32),
}

pub struct RoleCondFormulaValue {
    pub id: String,
    pub r#type: Box<RoleCondTypes>,
}

pub struct RoleLite {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub iconUrl: Option<UrlStr>,
    pub description: String,
    pub isModerator: bool,
    pub isAdministrator: bool,
    pub displayOrder: u8,
}

pub struct Role {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub iconUrl: Option<UrlStr>,
    pub description: String,
    pub isModerator: bool,
    pub isAdministrator: bool,
    pub displayOrder: u8,

    pub createdAt: String, // datetime
    pub updatedAt: String, // datetime
    pub target: RoleTarget,
    pub condFormula: RoleCondFormulaValue,

    pub isPublic: bool,
    pub isExplorable: bool,
    pub asBadge: bool,
    pub canEditMembersByModerator: bool,

    pub policies: Vec<HashMap<String, RolePoliciesTiny>>,
    pub usersCount: u16,
}

pub enum RoleTarget {
    manual,
    conditional,
}

pub enum NumberOrBool {
    number(u32),
    bool(bool),
}

pub struct RolePoliciesTiny {
    pub value: Option<NumberOrBool>,
    pub priority: Option<u8>,
    pub useDefault: Option<bool>,
}

pub struct RolePolicies {
    // this is "copy"
    pub ltlAvaliable: bool,
    pub gtlAvaliable: bool,
    pub canPublicNote: bool,
    pub mentionLimit: bool,
    pub canInvite: bool,
    pub inviteLimit: u64,
    pub inviteLimitCycle: u64,     // unixtime
    pub inviteExpirationTime: u64, // unixtime

    pub canManageCustomEmojis: bool,
    pub canManageAvatarDecorations: bool,
    pub canSearchNotes: bool,
    pub canUseTranslator: bool,
    pub canHideAds: bool,

    pub driveCapacityMb: u32,
    pub alwaysMarkNsfw: bool,
    pub pinLimit: u8,
    pub antennaLimit: u16,
    pub wordMuteLimit: u32,
    pub webhookLimit: u32,
    pub clipLimit: u32,

    pub noteEachClipsLimit: u32,
    pub userListLimit: u8,
    pub userEachUserListsLimit: u16,

    pub rateLimitFactor: u8,       // pcnt
    pub avatarDecorationLimit: u8, // more than 200 is it "legal" anyways?
}

pub struct ReversiGameLite {
    pub id: String,
    pub createdAt: String, // datetime
    pub startedAt: Option<String>,
    pub endedAt: Option<String>,
    pub isStarted: bool,
    pub isEnded: bool,

    pub user1Id: String,
    pub user2Id: String,
    pub user1: UserLite,
    pub user2: UserLite,
    pub winnerId: Option<String>,
    pub winner: Option<UserLite>,
    pub surrenderedUserId: Option<String>,
    pub timeoutUserId: Option<String>,

    pub black: Option<u16>,
    pub bw: String,
    pub noIrregularRules: bool,
    pub isLlotheo: bool,
    pub canPutEverywhere: bool,
    pub loopedBoard: bool,
    pub timeLimitForEachTurn: u8,
}

pub struct ReversiGameDetailed {
    pub id: String,
    pub createdAt: String, // datetime
    pub startedAt: Option<String>,
    pub endedAt: Option<String>,
    pub isStarted: bool,
    pub isEnded: bool,

    pub form1: Option<HashMap<String, !>>,
    pub form2: Option<HashMap<String, !>>,
    pub user1Ready: bool,
    pub user2Ready: bool,

    pub user1Id: String,
    pub user2Id: String,
    pub user1: UserLite,
    pub user2: UserLite,
    pub winnerId: Option<String>,
    pub winner: Option<UserLite>,
    pub surrenderedUserId: Option<String>,
    pub timeoutUserId: Option<String>,

    pub black: Option<u16>,
    pub bw: String,
    pub noIrregularRules: bool,
    pub isLlotheo: bool,
    pub canPutEverywhere: bool,
    pub loopedBoard: bool,
    pub timeLimitForEachTurn: u8,

    pub logs: Vec<Vec<u8>>,
    pub map: Vec<String>,
}

pub struct AdMeta {
    pub id: String,
    pub url: UrlStr,
    pub place: String,
    pub ratio: String,
    pub imageUrl: UrlStr,
    pub dayOfWeek: u8,
}

pub struct MetaLite {
    pub maintainerName: Option<String>,
    pub maintainerEmail: Option<String>,

    pub version: String,
    pub prividesTarball: bool,
    pub name: Option<String>,
    pub shortName: Option<String>,

    pub uri: UrlStr,
    pub description: Option<String>,
    pub langs: Vec<String>,
    pub tosUrl: Option<UrlStr>,
    pub repositiryUrl: Option<UrlStr>,
    pub feedbackUrl: Option<UrlStr>,

    pub defaultDarkTheme: Option<String>,
    pub defaultLightTheme: Option<String>,

    pub disableRegistration: bool,
    pub emailRequiredForSignup: bool,

    pub enableHcaptcha: bool,
    pub hcaptchaSiteKey: Option<String>,
    pub enableMcaptcha: bool,
    pub mcaptchaSiteKey: Option<String>,
    pub mcaptchaInstanceUrl: Option<UrlStr>,
    pub enableRecaptcha: bool,
    pub recaptchaSiteKey: Option<String>,
    pub enableTurnstile: bool,
    pub turnstileSiteKey: Option<String>,
    pub swPublickey: Option<String>,

    pub mascotImageUrl: UrlStr,
    pub bannerUrl: Option<UrlStr>,
    pub serverErrorImageUrl: Option<UrlStr>,
    pub infoImageUrl: Option<UrlStr>,
    pub notFoundImageUrl: Option<UrlStr>,
    pub iconUrl: Option<UrlStr>,

    pub maxNoteTextLength: u16,
    pub ads: Vec<AdMeta>,
    pub notesPerOneAd: u16,

    pub enableEmail: bool,
    pub enableServiceWorker: bool,
    pub translatorAvaliable: bool,

    pub mediaProxy: UrlStr,
    pub enableUrlPreview: bool,
    pub backgroundImageUrl: Option<UrlStr>,
    pub impressumUrl: Option<UrlStr>,
    pub logoImageUrl: Option<UrlStr>,
    pub privacyPolicyUrl: Option<UrlStr>,
    pub inquiryUrl: Option<UrlStr>,

    pub serverRules: Vec<String>,
    pub themeColor: Option<String>,
    pub policies: RolePolicies,
}

pub struct FeaturesMeta {
    // copy?
    pub registration: bool,
    pub emailRequiredForSignup: bool,
    pub localTimeline: bool,
    pub globalTimeline: bool,
    pub hcaptcha: bool,
    pub turnstile: bool,
    pub recaptcha: bool,
    pub objectStorage: bool,
    pub serviceWorker: bool,
    pub miauth: Option<bool>,
}

pub struct MetaDetailedOnly {
    pub features: Option<FeaturesMeta>,
    pub proxyAccountName: Option<String>,
    pub requireSetup: bool,
    pub cacheRemoteFiles: bool,
    pub cacheRemoteSensitiveFiles: bool,
}

pub struct MetaDetailed {
    pub maintainerName: Option<String>,
    pub maintainerEmail: Option<String>,

    pub version: String,
    pub prividesTarball: bool,
    pub name: Option<String>,
    pub shortName: Option<String>,

    pub uri: UrlStr,
    pub description: Option<String>,
    pub langs: Vec<String>,
    pub tosUrl: Option<UrlStr>,
    pub repositiryUrl: Option<UrlStr>,
    pub feedbackUrl: Option<UrlStr>,

    pub defaultDarkTheme: Option<String>,
    pub defaultLightTheme: Option<String>,

    pub disableRegistration: bool,
    pub emailRequiredForSignup: bool,

    pub enableHcaptcha: bool,
    pub hcaptchaSiteKey: Option<String>,
    pub enableMcaptcha: bool,
    pub mcaptchaSiteKey: Option<String>,
    pub mcaptchaInstanceUrl: Option<UrlStr>,
    pub enableRecaptcha: bool,
    pub recaptchaSiteKey: Option<String>,
    pub enableTurnstile: bool,
    pub turnstileSiteKey: Option<String>,
    pub swPublickey: Option<String>,

    pub mascotImageUrl: UrlStr,
    pub bannerUrl: Option<UrlStr>,
    pub serverErrorImageUrl: Option<UrlStr>,
    pub infoImageUrl: Option<UrlStr>,
    pub notFoundImageUrl: Option<UrlStr>,
    pub iconUrl: Option<UrlStr>,

    pub maxNoteTextLength: u16,
    pub ads: Vec<AdMeta>,
    pub notesPerOneAd: u16,

    pub enableEmail: bool,
    pub enableServiceWorker: bool,
    pub translatorAvaliable: bool,

    pub mediaProxy: UrlStr,
    pub enableUrlPreview: bool,
    pub backgroundImageUrl: Option<UrlStr>,
    pub impressumUrl: Option<UrlStr>,
    pub logoImageUrl: Option<UrlStr>,
    pub privacyPolicyUrl: Option<UrlStr>,
    pub inquiryUrl: Option<UrlStr>,

    pub serverRules: Vec<String>,
    pub themeColor: Option<String>,
    pub policies: RolePolicies,

    pub features: Option<FeaturesMeta>,
    pub proxyAccountName: Option<String>,
    pub requireSetup: bool,
    pub cacheRemoteFiles: bool,
    pub cacheRemoteSensitiveFiles: bool,
}

pub enum SysWebhookOn {
    abuseReport,
    abuseReportResolved,
}

pub struct SystemWebhook {
    pub id: String,
    pub isActive: bool,

    pub updatedAt: bool,
    pub latestSentAt: Option<String>, // @nullable datetime
    pub latestStatus: Option<u16>,    // HTTPreturnCode

    pub name: String,
    pub on: Vec<SysWebhookOn>,
    pub url: UrlStr,
    pub secret: String,
}

pub enum AbuseReportNotiMethod {
    email,
    webhook,
}

pub struct AbuseReportNotificationRecipient {
    pub id: String,
    pub isActive: bool,
    pub updatedAt: String,
    pub name: String,

    pub method: AbuseReportNotiMethod,
    pub userId: Option<String>,
    pub user: Option<UserLite>,
    pub systemWebhookId: Option<String>,
    pub systemWebhook: Option<SystemWebhook>,
}

// nevers
pub type responses = !;
pub type parameters = !;
pub type requestBodies = !;
pub type headers = !;
pub type pathItems = !;
