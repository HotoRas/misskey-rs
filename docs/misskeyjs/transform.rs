use std::collections::HashMap;

pub type UrlStr = String;

pub struct EError {
    pub code: String;
    pub message: String;
    pub id: String;
}

pub struct Error {
    pub error: EError;
}

pub struct AvatarDecoration {
    pub id: string;
    pub mut angle: Option<i16>;
    pub mut flipH: Option<bool>;
    pub mut url: UrlStr;
}

pub struct Instance {
    pub mut name: Option<String>;
    pub mut softwareName: Option<String>;
    pub mut softwareVersion: Option<String>;
    pub mut iconUrl: Option<UrlStr>;
    pub mut faviconUrl: Option<UrlStr>;
    pub mut themeColor: Option<String>;
}

pub type Emojis = HashMap<String, String>;

pub enum OnlineStatus {
    UNKNOWN,
    ONLINE,
    ACTIVE,
    OFFLINE,
}

pub struct BadgeRole {
    pub mut name: String;
    pub mut iconUrl: Option<UrlStr>;
    pub mut displayOrder: u16;
    pub mut behavior: Option<String>;
}

pub struct UserLite {
    pub id: String;
    pub mut name: Option<String>;
    pub username: String;
    pub host: Option<String>;
    pub mut avatarUrl: Option<UrlStr>;
    pub mut avatarBlurhash: Option<UrlStr>;

    pub mut avatarDecorations: Vec<AvatarDecoration>;
    pub mut isBot: Option<bool>;
    pub mut isCat: Option<bool>;
    pub instance: Option<Instance>;

    pub mut emojis: Emojis;
    pub mut onlineStatus: OnlineStatus;
    pub mut badgeRoles: Vec<BadgeRole>;
}

pub type User = UserLite;

pub struct UserFields {
    pub mut name: String;
    pub mut value: String;
}

pub struct Visibility {
    PUBLIC,
    FOLLOWERS,
    PRIVATE,
}

pub struct NotifyConf {
    NORMAL,
    NONE,
}

pub struct UserDetailedNotMe {
    pub id: String;
    pub mut name: Option<String>;
    pub username: String;
    pub host: Option<String>;
    pub mut avatarUrl: Option<UrlStr>;
    pub mut avatarBlurhash: Option<UrlStr>;

    pub mut avatarDecorations: Vec<AvatarDecoration>;
    pub mut isBot: Option<bool>;
    pub mut isCat: Option<bool>;
    pub instance: Option<Instance>;

    pub mut emojis: Emojis;
    pub mut onlineStatus: OnlineStatus;
    pub mut badgeRoles: Vec<BadgeRole>;

    pub mut url: Option<UrlStr>;
    pub mut uri: Option<UrlStr>;
    pub mut movedTo: Option<UrlStr>;
    pub mut alsoKnownAs: Option<Vec<String>>;

    pub createdAt: String; // -> chrono::DateTime
    pub mut updatedAt: Option<String>; // -> chrono::DateTime
    pub mut lastFetchedAt: Option<String>; // -> chrono::DateTime

    pub mut bannerUrl: Option<UrlStr>;
    pub mut bannerBlurhash: Option<String>;

    pub mut isLocked: bool;
    pub mut isSilenced: bool;
    pub mut isLimited: bool;
    pub mut isSuspended: bool;

    pub mut description: Option<String>;
    pub mut location: Option<String>;
    pub mut birthday: Option<String>; // YYYY-MM-DD
    pub mut lang: Option<String>;

    pub mut fields: Vec<UserFields>;
    pub mut verifiedLinks: Vec<UrlStr>;
    pub mut followersCount: u32;
    pub mut followingCount: u32;
    pub mut notesCount: u32;
    
    pub mut pinnedNoteIds: Vec<String>;
    pub mut pinnedNotes: Vec<Note>;
    pub mut pinnedPageId: Option<String>;
    pub mut pinnedPage: Option<Page>;

    pub mut publicReactions: bool;
    pub mut followingVisibility: Visibility;
    pub mut followersVisibility: Visibility;

    pub mut twoFactorEnabled: bool;
    pub mut usePasswordLessLogin: bool;
    pub mut securityKeys: bool;

    pub mut roles: Vec<RoleLite>;
    pub mut memo: Option<String>;
    pub mut moderationNote: Option<String>;
    
    pub mut isFollowing: Option<bool>;
    pub mut isFollowed: Option<bool>;
    pub mut hasPendingFollowRequestFromYou: Option<bool>;
    pub mut hasPendingFollowRequestToYou: Option<bool>;

    pub mut isBlocking: Option<bool>;
    pub mut isBlocked: Option<bool>;
    pub mut isMuted: Option<bool>;
    pub mut isRenoteMuted: Option<bool>;

    pub mut notify: NotifyConf;
    pub mut withReplies: Option<bool>;
}

pub type UserDetailed = UserDetailedNotMe;

pub enum TwoFactorBackupStock {
    FULL,
    PARTIAL,
    NONE,
}

pub enum OneOf<T> {
    One(T),
}

pub enum NotifyFrom {
    ALL,
    FOLLOWING,
    FOLLOWER,
    MUTURALFOLLOW,
    FOLLOWINGORFOLLOWER,
    NEVER,
    LIST(String),
}

pub type NotifyFromOneOf = OneOf<NotifyFrom>;

pub struct NotificationRecieveConf {
    pub mut note: Option<NotifyFromOneOf>;
    pub mut follow: Option<NotifyFromOneOf>;
    pub mut mention: Option<NotifyFromOneOf>;
    pub mut reply: Option<NotifyFromOneOf>;
    pub mut renote: Option<NotifyFromOneOf>;
    pub mut quote: Option<NotifyFromOneOf>;
    pub mut reaction: Option<NotifyFromOneOf>;
    pub mut pollEnded: Option<NotifyFromOneOf>;
    pub mut receiveFollowRequest: Option<NotifyFromOneOf>;
    pub mut followRequestAccepted: Option<NotifyFromOneOf>;
    pub mut roleAssigned: Option<NotifyFromOneOf>;
    pub mut achivementEarned: Option<NotifyFromOneOf>;
    pub mut app: Option<NotifyFromOneOf>;
    pub mut test: Option<NotifyFromOneOf>;
}

pub type Achivements = HashMap<String, i64>; // name, timestamp

pub enum SecurityKeysList {
    pub id: String;
    pub mut name: String;
    pub mut lastUsed: String;
}

pub struct MeDetailed {
    pub id: String;
    pub mut name: Option<String>;
    pub username: String;
    pub host: Option<String>;
    pub mut avatarUrl: Option<UrlStr>;
    pub mut avatarBlurhash: Option<UrlStr>;

    pub mut avatarDecorations: Vec<AvatarDecoration>;
    pub mut isBot: Option<bool>;
    pub mut isCat: Option<bool>;
    pub instance: Option<Instance>;

    pub mut emojis: Emojis;
    pub mut onlineStatus: OnlineStatus;
    pub mut badgeRoles: Vec<BadgeRole>;

    pub mut url: Option<UrlStr>;
    pub mut uri: Option<UrlStr>;
    pub mut movedTo: Option<UrlStr>;
    pub mut alsoKnownAs: Option<Vec<String>>;

    pub createdAt: String; // -> chrono::DateTime
    pub mut updatedAt: Option<String>; // -> chrono::DateTime
    pub mut lastFetchedAt: Option<String>; // -> chrono::DateTime

    pub mut bannerUrl: Option<UrlStr>;
    pub mut bannerBlurhash: Option<String>;

    pub mut isLocked: bool;
    pub mut isSilenced: bool;
    pub mut isLimited: bool;
    pub mut isSuspended: bool;

    pub mut description: Option<String>;
    pub mut location: Option<String>;
    pub mut birthday: Option<String>; // YYYY-MM-DD
    pub mut lang: Option<String>;

    pub mut fields: Vec<UserFields>;
    pub mut verifiedLinks: Vec<UrlStr>;
    pub mut followersCount: u32;
    pub mut followingCount: u32;
    pub mut notesCount: u32;
    
    pub mut pinnedNoteIds: Vec<String>;
    pub mut pinnedNotes: Vec<Note>;
    pub mut pinnedPageId: Option<String>;
    pub mut pinnedPage: Option<Page>;

    pub mut publicReactions: bool;
    pub mut followingVisibility: Visibility;
    pub mut followersVisibility: Visibility;

    pub mut twoFactorEnabled: bool;
    pub mut usePasswordLessLogin: bool;
    pub mut securityKeys: bool;

    pub mut roles: Vec<RoleLite>;
    pub mut memo: Option<String>;
    pub mut moderationNote: Option<String>;
    
    pub mut isFollowing: Option<bool>;
    pub mut isFollowed: Option<bool>;
    pub mut hasPendingFollowRequestFromYou: Option<bool>;
    pub mut hasPendingFollowRequestToYou: Option<bool>;

    pub mut isBlocking: Option<bool>;
    pub mut isBlocked: Option<bool>;
    pub mut isMuted: Option<bool>;
    pub mut isRenoteMuted: Option<bool>;

    pub mut notify: NotifyConf;
    pub mut withReplies: Option<bool>;

    pub mut avatarId: Option<String>;
    pub mut bannerId: Option<String>;
    pub mut isModerator: Option<bool>;
    pub mut isAdmin: Option<bool>;

    pub mut injectFeaturedNote: bool;
    pub mut recieveAnnouncementEmail: bool;
    pub mut alwaysMarkNsfw: bool;
    pub mut autoSensitive: bool;

    pub mut carefulBot: bool;
    pub mut autoAcceptFollowed: bool;
    pub mut noCrawle: bool;
    pub mut preventAiLearning: bool;
    pub mut isExplorable: bool;
    pub mut isDeleted: bool;

    pub mut twoFactorBackupCodesStock: TwoFactorBackupStock;
    pub mut hideOnlineStatus: bool;
    pub mut hasUnreadSpecifiedNotes: bool;
    pub mut hasUnreadMentions: bool;
    pub mut unreadAnnouncements: Vec<Announcement>;
    pub mut hasUnreadAntenna: bool;
    pub mut hasUnreadChannel: bool;
    pub mut hasUnreadNotification: bool;
    pub mut hadPendingRecievedFollowRequest: bool;
    pub mut unreadNotificationsCount: u16;
    pub mut mutedWords: Vec<Vec<String>>;
    pub mut mutedInstances: Option<Vec<UrlStr>>;

    pub mut notificationRecieveConfig: NotificationRecieveConf;
    pub mut emailNotificationTypes: Vec<String>;
    pub mut achivements: Achivements;
    pub mut loggedInDays: u32;
    pub mut policies: RolePolicies;
    
    pub mut email: Option<String>; // email
    pub mut emailVerified: Option<bool>;
    pub mut securityKeysList: Option<Vec<SecurityKeysList>>;
    pub mut token: Option<String>;
}
//TODO User까지 완료하긴 함.. 일단은.