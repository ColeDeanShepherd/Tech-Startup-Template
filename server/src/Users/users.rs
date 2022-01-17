type UserId = u64;

struct User {
    id: UserId,
    username: String,
    email: String,
    name: String,
    bio: String,
    created_at: Instant
}

struct SignUpAction {}
struct DeleteUserAction {}
struct ChangeUsernameAction {}
struct ChangePasswordAction {}
struct ChangeBioAction {}
struct ChangeProfilePictureAction {}
struct FollowUserAction {}
struct UnfollowUserAction {}
struct BlockUserAction {}