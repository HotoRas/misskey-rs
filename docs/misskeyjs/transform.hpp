#include <string>
#include <map>
#include <vector>

// decorator defines
#define NS_MK       \
  namespace Misskey \
  {
#define NS_MK_END }

#define NS_MK_JS      \
  namespace MisskeyJS \
  {
#define NS_MK_JS_END }

// type extension defines
typedef std::string Url;

// type shorthand defines
#define string std::string

NS_MK NS_MK_JS

    struct _error
{
  string code;
  string message;
  string id;
};

struct Error
{
  _error error;
};

struct AvatarDecoration
{
  string id;
  _Float64 angle;
  bool flipH;
  string url;
  int8_t offsetX;
  int8_t offsetY;
};

struct Instance
{
  string name;
  string softwareName;
  string softwareVersion;
  string iconUrl;
  string faviconUrl;
  string themeColor;
};

typedef std::map<string, string> Emojis;

enum OnlineStatus
{
  UNKNOWN,
  ONLINE,
  ACTIVE,
  OFFLINE,
};

struct BadgeRole
{
  string name;
  Url iconUrl;
  int displayOrder;
  string behavior;
};

struct UserLite
{
  string id;
  string name;
  string username;
  string host;
  Url avatarUrl;
  string avatarBlurHash;
  std::vector<AvatarDecoration> avatarDecorations;
  bool isBot;
  bool isCat;
  Instance instance;
  Emojis emojis;
  OnlineStatus onlineStatus;
  std::vector<BadgeRole> badgeRoles;
};

struct _field
{
  string name;
  string value;
};

typedef Note;
typedef Page;
enum Visibility
{
  PUBLIC,
  FOLLOWERS,
  PRIVATE,
};
typedef RoleLite;

enum _notify
{
  NORMAL,
  NONE,
};

struct UserDetailedNotMeOnly
{
  Url url;
}

NS_MK_JS_END NS_MK_END

#undef string