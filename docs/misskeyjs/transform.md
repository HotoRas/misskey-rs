# Transform

## components
on src/autogen/types > components:

### schemas

#### Error
```ts
type Error = {
  error: {
    code: string;
    message: string;
    id: string;
  };
}
```

```cpp
struct _error {
  std::string code;
  std::string message;
  std::string id;
};

struct Error {
  _error error;
};
```

```rs
pub struct ErrorError {
  pub code: String;
  pub message: String;
  pub id: String;
}

struct Error {
  pub error: ErrorError;
}
```

#### User
```ts
type UserLite = {
  id: string;
  name: string | null;
  username: string;
  host: string | null;
  avatarUrl: string | null;
  avatarBlurhash: string | null;
  avatarDecorations: {
    id: string;
    angle?: number;
    flipH?: boolean;
    url: string;
    offsetX?: number;
    offsetY?: number;
  }[];
  isBot?: boolean;
  isCat?: boolean;
  instance?: {
    name: string | null;
    softwareName: string | null;
    softwareVersion: string | null;
    iconUrl: string | null;
    faviconUrl: string | null;
    themeColor: string | null;
  };
  emojis: {
    [key: string]: string;
  };
  onlineStatus: 'unknown' | 'online' | 'active' | 'offline';
  badgeRoles?: ({
      name: string;
      iconUrl: string | null;
      displayOrder: number;
      behavior?: string;
  })[];
};
type UserDetailedNotMeOnly = {
  /** Format: url */
  url: string | null;
  /** Format: uri */
  uri: string | null;
  /** Format: uri */
  movedTo: string | null;
  alsoKnownAs: string[] | null;
  /** Format: date-time */
  createdAt: string;
  /** Format: date-time */
  updatedAt: string | null;
  /** Format: date-time */
  lastFetchedAt: string | null;
  /** Format: url */
  bannerUrl: string | null;
  bannerBlurhash: string | null;
  isLocked: boolean;
  isSilenced: boolean;
  isLimited: boolean;
  /** @example false */
  isSuspended: boolean;
  /** @example Hi masters, I am Ai! */
  description: string | null;
  location: string | null;
  /** @example 2018-03-12 */
  birthday: string | null;
  /** @example ja-JP */
  lang: string | null;
  fields: {
      name: string;
      value: string;
    }[];
  verifiedLinks: string[];
  followersCount: number;
  followingCount: number;
  notesCount: number;
  pinnedNoteIds: string[];
  pinnedNotes: components['schemas']['Note'][];
  pinnedPageId: string | null;
  pinnedPage: components['schemas']['Page'] | null;
  publicReactions: boolean;
  /** @enum {string} */
  followingVisibility: 'public' | 'followers' | 'private';
  /** @enum {string} */
  followersVisibility: 'public' | 'followers' | 'private';
  /** @default false */
  twoFactorEnabled: boolean;
  /** @default false */
  usePasswordLessLogin: boolean;
  /** @default false */
  securityKeys: boolean;
  roles: components['schemas']['RoleLite'][];
  memo: string | null;
  moderationNote?: string;
  isFollowing?: boolean;
  isFollowed?: boolean;
  hasPendingFollowRequestFromYou?: boolean;
  hasPendingFollowRequestToYou?: boolean;
  isBlocking?: boolean;
  isBlocked?: boolean;
  isMuted?: boolean;
  isRenoteMuted?: boolean;
  /** @enum {string} */
  notify?: 'normal' | 'none';
  withReplies?: boolean;
};
type MeDetailedOnly = {
  /** Format: id */
  avatarId: string | null;
  /** Format: id */
  bannerId: string | null;
  isModerator: boolean | null;
  isAdmin: boolean | null;
  injectFeaturedNote: boolean;
  receiveAnnouncementEmail: boolean;
  alwaysMarkNsfw: boolean;
  autoSensitive: boolean;
  carefulBot: boolean;
  autoAcceptFollowed: boolean;
  noCrawle: boolean;
  preventAiLearning: boolean;
  isExplorable: boolean;
  isDeleted: boolean;
  /** @enum {string} */
  twoFactorBackupCodesStock: 'full' | 'partial' | 'none';
  hideOnlineStatus: boolean;
  hasUnreadSpecifiedNotes: boolean;
  hasUnreadMentions: boolean;
  hasUnreadAnnouncement: boolean;
  unreadAnnouncements: components['schemas']['Announcement'][];
  hasUnreadAntenna: boolean;
  hasUnreadChannel: boolean;
  hasUnreadNotification: boolean;
  hasPendingReceivedFollowRequest: boolean;
  unreadNotificationsCount: number;
  mutedWords: string[][];
  mutedInstances: string[] | null;
  notificationRecieveConfig: {
    note?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    follow?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    mention?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    reply?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    renote?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    quote?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    reaction?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    pollEnded?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    receiveFollowRequest?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    followRequestAccepted?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    roleAssigned?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    achievementEarned?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    app?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
    test?: OneOf<[{
      /** @enum {string} */
      type: 'all' | 'following' | 'follower' | 'mutualFollow' | 'followingOrFollower' | 'never';
    }, {
      /** @enum {string} */
      type: 'list';
      /** Format: misskey:id */
      userListId: string;
    }]>;
  };
  emailNotificationTypes: string[];
  achievements: {
      name: string;
      unlockedAt: number;
    }[];
  loggedInDays: number;
  policies: components['schemas']['RolePolicies'];
  email?: string | null;
  emailVerified?: boolean | null;
  securityKeysList?: {
      /**
       * Format: id
       * @example xxxxxxxxxx
       */
      id: string;
      name: string;
      /** Format: date-time */
      lastUsed: string;
    }[];
  token?: string;
};
type UserDetailedNotMe = UserLite & UserDetailedNotMeOnly;
type MeDetailed = UserLite & UserDetailedNotMeOnly & MeDetailedOnly;
type UserDetailed = UserDetailedNotMe | MeDetailed;
type User = UserLite | UserDetailed;
```

```c++
using std::string;
struct AvatarDecorations {
  string id;
  float64_t? angle;
  bool? flipH;
  string url;
  int? offsetX;
  int? offsetY;
};
```
```c++
struct instance
```
```c++
struct UserLite {
  string id;
  string? name;
  string username;
  string? avatarUrl;
  string? avatarBlurhash;
  std::vector<AvatarDecorations> avatarDecorations;
};