#![allow(non_upper_case_globals, non_camel_case_types)]

pub const notificationTypes: [&str; 15] = [
    "note",
    "follow",
    "mention",
    "reply",
    "renote",
    "quote",
    "reaction",
    "pollVote",
    "pollEnded",
    "receiveFollowRequest",
    "followRequestAccepted",
    "groupInvited",
    "app",
    "roleAssigned",
    "achievementEarned",
];

pub const noteVisibilities: [&str; 4] = ["public", "home", "followers", "specified"];

pub const mutedNoteReasons: [&str; 4] = ["word", "manual", "spam", "other"];

pub const followingVisibilities: [&str; 3] = ["public", "followers", "private"];

pub const followersVisibilities: [&str; 3] = followingVisibilities;

pub const permissions: [&str; 88] = [
    "read:account",
    "write:account",
    "read:blocks",
    "write:blocks",
    "read:drive",
    "write:drive",
    "read:favorites",
    "write:favorites",
    "read:following",
    "write:following",
    "read:messaging",
    "write:messaging",
    "read:mutes",
    "write:mutes",
    "write:notes",
    "read:notifications",
    "write:notifications",
    "read:reactions",
    "write:reactions",
    "write:votes",
    "read:pages",
    "write:pages",
    "write:page-likes",
    "read:page-likes",
    "read:user-groups",
    "write:user-groups",
    "read:channels",
    "write:channels",
    "read:gallery",
    "write:gallery",
    "read:gallery-likes",
    "write:gallery-likes",
    "read:flash",
    "write:flash",
    "read:flash-likes",
    "write:flash-likes",
    "read:admin:abuse-user-reports",
    "read:admin:abuse-report-resolvers",
    "write:admin:abuse-report-resolvers",
    "read:admin:index-stats",
    "read:admin:table-stats",
    "read:admin:user-ips",
    "read:admin:meta",
    "write:admin:reset-password",
    "write:admin:resolve-abuse-user-report",
    "write:admin:send-email",
    "read:admin:server-info",
    "read:admin:show-moderation-log",
    "read:admin:show-user",
    "read:admin:show-users",
    "write:admin:suspend-user",
    "write:admin:unset-user-avatar",
    "write:admin:unset-user-banner",
    "write:admin:unsuspend-user",
    "write:admin:meta",
    "write:admin:user-note",
    "write:admin:roles",
    "read:admin:roles",
    "write:admin:relays",
    "read:admin:relays",
    "write:admin:invite-codes",
    "read:admin:invite-codes",
    "write:admin:announcements",
    "read:admin:announcements",
    "write:admin:avatar-decorations",
    "read:admin:avatar-decorations",
    "write:admin:federation",
    "write:admin:indie-auth",
    "read:admin:indie-auth",
    "write:admin:account",
    "read:admin:account",
    "write:admin:emoji",
    "read:admin:emoji",
    "write:admin:queue",
    "read:admin:queue",
    "write:admin:promo",
    "write:admin:drive",
    "read:admin:drive",
    "write:admin:sso",
    "read:admin:sso",
    "write:admin:ad",
    "read:admin:ad",
    "write:invite-codes",
    "read:invite-codes",
    "write:clip-favorite",
    "read:clip-favorite",
    "read:federation",
    "write:report-abuse",
];

pub const moderationLogTypes: [&str; 44] = [
    "updateServerSettings",
    "suspend",
    "unsuspend",
    "updateUserNote",
    "addCustomEmoji",
    "updateCustomEmoji",
    "depub consteCustomEmoji",
    "assignRole",
    "unassignRole",
    "createRole",
    "updateRole",
    "depub consteRole",
    "clearQueue",
    "promoteQueue",
    "depub consteDriveFile",
    "depub consteNote",
    "createGlobalAnnouncement",
    "createUserAnnouncement",
    "updateGlobalAnnouncement",
    "updateUserAnnouncement",
    "depub consteGlobalAnnouncement",
    "depub consteUserAnnouncement",
    "resetPassword",
    "suspendRemoteInstance",
    "unsuspendRemoteInstance",
    "updateRemoteInstanceNote",
    "markSensitiveDriveFile",
    "unmarkSensitiveDriveFile",
    "resolveAbuseReport",
    "createInvitation",
    "createAd",
    "updateAd",
    "depub consteAd",
    "createIndieAuthClient",
    "updateIndieAuthClient",
    "depub consteIndieAuthClient",
    "createSSOServiceProvider",
    "updateSSOServiceProvider",
    "depub consteSSOServiceProvider",
    "createAvatarDecoration",
    "updateAvatarDecoration",
    "depub consteAvatarDecoration",
    "unsetUserAvatar",
    "unsetUserBanner",
];

pub mod ModerationLogPayloads {
    use std::collections::HashMap;

    use json::JsonValue;

    pub struct updateServerSettings {
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub struct suspend {
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
    }

    pub type unsuspend = suspend;

    pub struct updateUserNote {
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
        pub before: Option<String>,
        pub after: Option<String>,
    }

    pub struct addCustomEmoji {
        pub emojiId: String,
        pub emoji: JsonValue,
    }

    pub struct updateCustomEmoji {
        pub emojiId: String,
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub type deleteCustomEmoji = addCustomEmoji;

    pub struct assignRole {
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
        pub roleId: String,
        pub roleName: String,
        pub expiresAt: Option<String>,
    }

    pub struct unassignRole {
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
        pub roleId: String,
        pub roleName: String,
    }

    pub struct createRole {
        pub roleId: String,
        pub role: JsonValue,
    }

    pub struct updateRole {
        pub roleId: String,
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub type deleteRole = createRole;

    pub type clearQueue = HashMap<String, !>;
    pub type promoteQueue = HashMap<String, !>;

    pub struct deleteDriveFile {
        pub fileId: String,
        pub fileUserId: Option<String>,
        pub fileUserUsername: Option<String>,
        pub fileUserHost: Option<String>,
    }

    pub struct deleteNote {
        pub noteId: String,
        pub noteUserId: String,
        pub noteUserUsername: String,
        pub noteUserHost: Option<String>,
        pub note: JsonValue,
    }

    pub struct createGlobalAnnouncement {
        pub announcementId: String,
        pub announcement: JsonValue,
    }

    pub struct createUserAnnouncement {
        pub announcementId: String,
        pub announcement: JsonValue,
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
    }

    pub struct updateGlobalAnnouncement {
        pub announcementId: String,
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub struct updateUserAnnouncement {
        pub announcementId: String,
        pub before: JsonValue,
        pub after: JsonValue,
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
    }

    pub type deleteGlobalAnnouncement = createGlobalAnnouncement;
    pub type deleteUserAnnouncement = createUserAnnouncement;

    pub struct resetPassword {
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
    }

    pub struct suspendRemoteInstance {
        pub id: String,
        pub host: String,
    }

    pub type unsuspendRemoteInstance = suspendRemoteInstance;

    pub struct updateRemoteInstanceNote {
        pub id: String,
        pub host: String,
        pub before: Option<JsonValue>,
        pub after: Option<JsonValue>,
    }

    pub struct markSensitiveDriveFile {
        pub fileId: String,
        pub fileUserId: Option<String>,
        pub fileUserUsername: Option<String>,
        pub fileUserHost: Option<String>,
    }

    pub type unmarkSensitiveDriveFile = markSensitiveDriveFile;

    pub struct resolveAbuseReport {
        pub reportId: String,
        pub report: JsonValue,
        pub forwarded: bool,
    }

    pub struct createInvitation {
        pub invitations: Vec<JsonValue>,
    }

    pub struct createAd {
        pub adId: String,
        pub ad: JsonValue,
    }

    pub struct updateAd {
        pub adId: String,
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub type deleteAd = createAd;

    pub struct createIndieAuthClient {
        pub clientId: String,
        pub client: JsonValue,
    }

    pub struct updateIndieAuthClient {
        pub clientId: String,
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub type deleteIndieAuthClient = createIndieAuthClient;

    pub struct createSSOServiceProvider {
        pub serviceId: String,
        pub service: JsonValue,
    }

    pub struct updateSSOServiceProvider {
        pub serviceId: String,
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub type deleteSSOServiceProvider = createSSOServiceProvider;

    pub struct createAvatarDecoration {
        pub avatarDecorationId: String,
        pub avatarDecoration: JsonValue,
    }

    pub struct updateAvatarDecoration {
        pub avatarDecorationId: String,
        pub before: JsonValue,
        pub after: JsonValue,
    }

    pub type deleteAvatarDecoration = createAvatarDecoration;

    pub struct unsetUserAvatar {
        pub userId: String,
        pub userUsername: String,
        pub userHost: Option<String>,
        pub fileId: String,
    }

    pub type unsetUserBanner = unsetUserAvatar;
}
