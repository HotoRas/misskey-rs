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

pub enum OnlineStatus { unknown, online, active, offline, }

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

pub struct Visibility { public, followers, private, }

pub struct NotifyConf { normal, none, }

pub struct UserDetailedNotMeOnly {
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

pub enum TwoFactorBackupStock { full, partial, none, }

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

pub struct SecurityKeysList {
    pub id: String;
    pub mut name: String;
    pub mut lastUsed: String;
}

pub struct MeDetailedOnly {
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

pub struct UserList {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut name: String;
    pub mut userIds: Option<String>;
    pub mut isPublic: bool;
}

pub struct Ad {
    pub id: String;
    pub mut expiresAt: String; // datetime
    pub mut startsAt: String; // datetime
    pub mut place: String;
    pub mut priority: String;
    pub mut ratio: u16;
    pub mut url: UrlStr;
    pub mut imageUrl: UrlStr;
    pub mut memo: String;
    pub mut dayOfWeek: u8;
}

pub enum AnnouncementIcon { info, warning, error, success, }

pub enum AnnouncementDisplay { dialog, normal, banner, }

pub struct Announcement {
    pub id: String;
    pub createdAt: String;
    pub mut updatedAt: Option<String>;
    pub mut text: String;
    pub mut title: String;
    pub mut imageUrl: Option<UrlStr>;
    pub mut icon: AnnouncementIcon;
    pub mut display: AnnouncmentDisplay;
    pub mut needConfirmationToRead: bool;
    pub mut silence: bool;
    pub mut forYou: bool;
    pub mut isRead: Option<bool>;
}

pub struct App {
    pub id: String;
    pub mut name: String;
    pub mut callbackUrl: Option<UrlStr>;
    pub mut permission: Vec<String>;
    pub mut secret: Option<String>;
    pub mut isAuthorized: Option<bool>;
}

pub enum NoteVisibility { public, home, followers, specified, }

pub struct NotePollChoice {
    pub mut isVoted: bool;
    pub mut text: String;
    pub mut votes: u32;
}

pub struct NotePoll {
    pub mut expiresAt: Option<String>; // datetime
    pub mut multiple: bool;
    pub mut choices: Vec<NotePollChoice>;
}

pub struct NoteChannel {
    pub id: String;
    pub mut name: String;
    pub mut color: String;
    pub mut isSensitive: bool;
    pub mut allowRenoteToExternal: bool;
    pub userId: Option<String>;
}

pub struct Note {
    pub id: String;
    pub createdAt: String; // datetime
    // pub mut editedAt: Option<String>; // datetime
    // pub mut editedAtHistory: Option<Vec<String>>; // datetime[]
    // pub mut editHistory: Option<Vec<String>>;
    pub mut deletedAt: Option<String>;

    pub mut text: Option<String>;
    pub mut cw: Option<String>;

    pub userId: String;
    pub mut user: UserLite;
    pub replyId: Option<String>;
    pub renoteId: Option<String>;
    pub reply: Option<Note>;
    pub renote: Option<Note>;

    pub mut isHidden: bool;
    pub mut visibility: NoteVisibility;
    
    pub mut mentions: Option<Vec<String>>;
    pub mut visibleUserIds: Option<Vec<String>>;

    pub mut fileIds: Option<Vec<String>>;
    pub mut files: Option<Vec<DriveFile>>;

    pub mut tags: Vec<String>;
    pub mut poll: Option<NotePoll>;

    pub mut emojis: Vec<HashMap<String, String>>;

    pub mut channelId: Option<String>;
    pub mut channel: Option<NoteChannel>;

    pub mut localOnly: Option<bool>;
    pub mut reactionAcceptance: Option<String>;
    pub mut reactionEmojis: Vec<HashMap<String, String>>;
    pub mut reactions: Vec<HashMap<String, u32>>;
    pub mut reactionCount: u32;
    pub mut renoteCount: u32;
    pub mut repliesCount: u32;

    pub mut uri: Option<UrlStr>;
    pub mut url: Option<UrlStr>;

    pub mut reactionAndUserPairCache: Option<Vec<String>>;
    pub mut clippedCount: Option<u32>;
    pub mut myReaction: Option<String>;
}

pub struct NoteReaction {
    pub id: String;
    pub createdAt: String;
    pub user: UserLite;
    pub type: String;
}

pub struct NoteFavorite {
    pub id: String;
    pub createdAt: String;
    pub note: Note;
    pub noteId: String;
}

pub enum Notification {
    note {
        id: String;
        createdAt: String;
        type: String = 'note';
        user: UserLite;
        userId: String;
        note: Note;
    },
    mention {
        id: string;
        createdAt: string;
        type: String = 'mention';
        user: UserLite;
        userId: string;
        note: Note;
    },
    renote {
        id: String;
        createdAt: String;
        type: String = 'renote';
        user: UserLite;
        userId: String;
        note: Note;
    },
    quote {
        id: String;
        createdAt: String;
        type: String = 'quote';
        user: UserLite;
        userId: String;
        note: Note;
    },
    reaction {
        id: String;
        createdAt: String;
        type: String = 'reaction';
        user: UserLite;
        userId: String;
        note: Note;
        reaction: String;
    },
    pollEnded {
        id: String;
        createdAt: String;
        type: String = 'pollEnded';
        user: UserLite;
        userId: String;
        note: Note;
    },
    follow {
        id: String;
        createdAt: String;
        type: String = 'follow';
        user: UserLite;
        userId: String;
    },
    followRequestAccepted {
        id: String;
        createdAt: String;
        type: String = 'followRequestAccepted';
        user: UserLite;
        userId: String;
    },
    roleAssigned {
        id: String;
        createdAt: String;
        type: String = 'roleAssigned';
        role: Role;
    },
    achivementEarned {
        id: String;
        createdAt: String;
        type: String = 'achivementEarned';
        achivement: String;
    },
    app {
        id: String;
        createdAt: String;
        type: String = 'app';
        body: String;
        header: String;
        icon: String;
    },
    reaction_Grouped {
        id: String;
        createdAt: String;
        type: String = "reaction:grouped";
        note: Note;
        reactions: Vec<NotifyReaction>;
    },
    renote_Grouped {
        id: String;
        createdAt: String;
        type: String = "renote:groupped";
        note: Note;
        users: Vec<UserLite>;
    },
    test {
        id: String;
        createdAt: String;
        type: String = 'test';
    }
}

pub struct NotifyReaction {
    pub user: UserLite;
    pub reaction: String;
}

pub struct DriveFolder {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut name: String;
    pub mut parentId: Option<String>;
    pub mut foldersCount: Option<u16>;
    pub mut filesCount: Option<u16>;
    pub mut parent: Option<DriveFolder>;
}

pub struct Following {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut followeeId: String;
    pub mut followerId: String;
    pub mut followee: Option<UserDetailedNotMe>;
    pub mut follower: Option<UserDetailedNotMe>;
}

pub struct Muting {
    pub id: String;
    pub createdAt: String;
    pub mut expiresAt: String;
    pub muteeId: String;
    pub mut mutee: UserDetailedNotMe;
}

pub struct RenoteMuting {
    pub id: String;
    pub createdAt: String;
    pub muteeId: String;
    pub mut mutee: UserDetailedNotMe;
}

pub struct blocking {
    pub id: String;
    pub createdAt: String;
    pub blockeeId: String;
    pub mut blockee: UserDetailedNotMe;
}

pub struct Hashtag {
    pub tag: String;
    pub mut mentionedUsersCount: u32;
    pub mut mentionedLocalUsersCount: u32;
    pub mut mentionedRemoteUsersCount: u32;
    pub mut attachedUsersCount: u32;
    pub mut attachedLocalUsersCount: u32;
    pub mut attachedRemoteUsersCount: u32;
}

pub struct InviteCode {
    pub id: String;
    pub code: String;
    pub expiresAt: Option<String>; // @nullable datetime
    pub createdAt: String; // datetime
    pub createdBy: Option<UserLite>;
    pub mut usedBy: Option<UserLite>;
    pub mut UsedAt: Option<String>; // datetime
    pub mut used: bool;
}

pub struct Page {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut updatedAt: String; // datetime
    pub userId: String;
    pub mut user: UserLite;
    pub mut content: Vec<PageBlock>;
    pub mut variables: Vec<HashMap<String, {UNKNOWN}>>;

    pub mut title: String;
    pub mut name: String;
    pub mut summary: Option<String>;
    pub mut hideTitleWhenPinned: bool;

    pub mut alignCenter: bool;
    pub mut font: String;
    pub mut script: String;
    pub mut eyeCatchingImageId: Option<String>;
    pub mut eyeCatchingImage: Option<DriveFile>;
    pub mut attachedFile: Vec<DriveFile>;

    pub mut likedCount: u16;
    pub mut isLiked: Option<bool>;
}

pub enum PageBlock {
    text {
        id: String;
        type: String = 'text';
        text: String;
    },
    section {
        id: String;
        type: String = 'sextion';
        title: String;
        children: Vec<PageBlock>;
    },
    image {
        id: String;
        type: String = 'image';
        fileId: Option<String>;
    },
    note {
        id: String;
        type: String = 'note';
        detailed: bool;
        note: Option<String>;
    }
}

pub struct Channel {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut lastNotedAt: Option<String>; // @nullable datetime
    pub mut name: String;
    pub mut description: Option<String>;
    pub mut bannerUrl: Option<UrlStr>;
    pub mut pinnedNoteIds: Vec<String>;
    pub mut color: String;
    pub mut isArchived: bool;
    pub mut usersCount: u16;
    pub mut notesCount: u32;
    pub mut isSensitive: bool;
    pub mut allowRenoteToExternal: bool;
    pub mut isFollowing: Option<bool>;
    pub mut isFavorited: Option<bool>;
    pub mut pinnedNotes: Option<Vec<Note>>;
}

pub struct QueueCount {
    // this is "copy"
    pub mut waiting: u32;
    pub mut active: u32;
    pub mut completed: u32;
    pub mut failed: u32;
    pub mut delayed: u32;
}

pub struct Antenna {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut name: String;
    pub mut keywords: Vec<Vec<String>>;
    pub mut excludeKeywords: Vec<Vec<String>>;
    pub mut src: AntennaSrc;
    pub mut userListId: Option<String>;
    pub mut users: Vec<String>;
    pub mut caseSensitive: bool;
    pub mut localOnly: bool;
    pub mut excludeBots: bool;
    pub mut withReplies: bool;
    pub mut withFile: bool;
    pub mut isActive: bool;
    pub mut hasUnreadNote: bool;
    pub mut notify: bool;
}

pub enum AntennaSrc {
    home, all, users, list, users_blacklist,
}

pub struct Clip {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut lastClippedAt: Option<String>; // @nullable datetime
    pub userId: String;
    pub mut user: UserLite;
    pub mut name: String;
    pub mut description: Option<String>;
    pub mut isPublic: bool;
    pub mut favoritedCount: u16;
    pub mut isFavorited: Option<bool>;
    pub mut notesCount: Option<u16>;
}

pub enum FederationSuspensionState {
    none, manuallySuspended, goneSuspended, autoSuspendedForNotResponding,
}

pub struct FederationInstance {
    pub id: String;
    pub firstRecievedAt: String; // datetime
    pub host: UrlStr;
    pub mut usersCount: u16;
    pub mut notesCount: u32;
    pub mut followingCount: u16;
    pub mut followersCount: u16;

    pub mut isNotResponding: bool;
    pub mut isSuspended: bool;
    pub mut suspensionState: FederationSuspensionState;

    pub mut isBlocked: bool;
    pub mut softwareName: Option<String>;
    pub mut softwareVersion: Option<String>;
    pub mut openRegistration: Option<bool>;

    pub mut name: Option<String>;
    pub mut description: Option<String>;
    pub mut maintainerName: Option<String>;
    pub mut maintainerEmail: Option<String>;
    pub mut isSilenced: bool;
    pub mut iconUrl: Option<UrlStr>;
    pub mut faviconUrl: Option<UrlStr>;
    pub mut themeColor: Option<String>;
    pub mut infoUpdatedAt: Option<String>; // @nullable datetime
    pub mut latestRequestRecievedAt: Option<String>; // @nullable datetime
    pub mut moderationNote: Option<String>;
}

pub struct GalleryPost {
    pub id: String;
    pub createdAt: String; // datetime
    pub mut updatedAt: String; // datetime
    pub userId: String;
    pub mut user: UserLite;
    pub mut title: String;
    pub mut description: Option<String>;
    pub mut fileIds: Option<Vec<String>>;
    pub mut files: Option<Vec<DriveFile>>;
    pub mut tags: Option<Vec<String>>;
    pub mut isSensitive: bool;
    pub mut likedCount: u16;
    pub mut isLiked: Option<bool>;
}

pub struct EmojiSimple {
    pub mut aliases: Vec<String>;
    pub mut name: String;
    pub mut category: Option<String>;
    pub mut url: UrlStr;
    pub mut localOnly: Option<bool>;
    pub mut isSensitive: Option<bool>;
    pub mut roleIdsThatCanBeUsedThisEmojiAsReaction: Option<Vec<String>>;
}

pub struct EmojiDetailed {
    pub id: String;
    pub mut aliases: Vec<String>;
    pub mut name: String;
    pub mut category: Option<String>;

    pub host: Option<String>;
    pub mut url: UrlStr;
    pub mut license: Option<String>;

    pub mut isSensitive: bool;
    pub mut localOnly: bool;
    pub mut roleIdsThatCanBeUsedThisEmojiAsReaction: Vec<String>;
}

pub struct Flash {
    pub id: String;
    pub createdAt: String;
    pub mut updatedAt: String;
    pub userId: String;
    pub mut user: UserLite;
    pub mut title: String;
    pub mut summary: String;
    pub mut script: String;
    pub mut likedCount: Option<u16>;
    pub mut isLiked: Option<bool>;
}

pub struct Signin {
    pub id: String;
    pub createdAt: String;
    pub ip: String;
    pub headers: HashMap<String, {{UNKNOWN}}>;
    pub success: bool;
}

pub enum RoleCondTypes {
    and(RoleCondFormulaValue), or(RoleCondFormulaValue), not(RoleCondFormulaValue),
    isLocal, isRemote,
    isSuspended, isLocked, isBot, isCat, isExplorable,
    roleAssignedTo(String),
    createdLessThan(u64), createdMoreThan(u64),
    followersLessThanOrEq(u32), followersMoreThanOrEq(u32),
    followingLessThanOrEq(u32), followingMoreThanOrEq(u32),
    notesLessThanOrEq(u32), notesMoreThanOrEq(u32),
}

pub struct RoleCondFormulaValue {
    pub id: String;
    pub mut type: RoleCondTypes;
}

pub struct RoleLite {
    pub id: String;
    pub mut name: String;
    pub mut color: Option<String>;
    pub mut iconUrl: Option<UrlStr>;
    pub mut description: String;
    pub mut isModerator: bool;
    pub mut isAdministrator: bool;
    pub mut displayOrder: u8;
}

pub struct Role {
    pub id: String;
    pub mut name: String;
    pub mut color: Option<String>;
    pub mut iconUrl: Option<UrlStr>;
    pub mut description: String;
    pub mut isModerator: bool;
    pub mut isAdministrator: bool;
    pub mut displayOrder: u8;

    pub createdAt: String; // datetime
    pub mut updatedAt: String; // datetime
    pub mut target: RoleTarget;
    pub mut condFormula: RoleCondFormulaValue;

    pub mut isPublic: bool;
    pub mut isExplorable: bool;
    pub mut asBadge: bool;
    pub mut canEditMembersByModerator: bool;

    pub mut policies: Vec<HashMap<String, RolePoliciesTiny>>;
    pub mut usersCount: u16;
}

pub enum RoleTarget { manual, conditional, }

pub enum NumberOrBool {
    number(u32), bool(bool),
}

pub struct RolePoliciesTiny {
    pub value: Option<NumberOrBool>;
    pub mut priority: Option<u8>;
    pub mut useDefault: Option<bool>;
}

pub struct RolePolicies {
    // this is "copy"
    pub mut ltlAvaliable: bool;
    pub mut gtlAvaliable: bool;
    pub mut canPublicNote: bool;
    pub mut mentionLimit: bool;
    pub mut canInvite: bool;
    pub mut inviteLimit: u64;
    pub mut inviteLimitCycle: u64; // unixtime
    pub mut inviteExpirationTime: u64; // unixtime

    pub mut canManageCustomEmojis: bool;
    pub mut canManageAvatarDecorations: bool;
    pub mut canSearchNotes: bool;
    pub mut canUseTranslator: bool;
    pub mut canHideAds: bool;

    pub mut driveCapacityMb: u32;
    pub mut alwaysMarkNsfw: bool;
    pub mut pinLimit: u8;
    pub mut antennaLimit: u16;
    pub mut wordMuteLimit: u32;
    pub mut webhookLimit: u32;
    pub mut clipLimit: u32;

    pub mut noteEachClipsLimit: u32;
    pub mut userListLimit: u8;
    pub mut userEachUserListsLimit: u16;
    
    pub mut rateLimitFactor: u8; // pcnt
    pub mut avatarDecorationLimit: u8; // more than 200 is it "legal" anyways?
}

//TODO start back from Reversi
//@src https://github.com/misskey-dev/misskey/blob/develop/packages/misskey-js/src/autogen/types.ts#L4800

// nevers
pub type responses: Never;
pub type parameters: Never;
pub type requestBodies: Never;
pub type headers: Never;
pub type pathItems: Never;