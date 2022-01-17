pub type UserId = u64;

pub struct User {
    id: UserId,
    username: String,
    email: String,
    name: String,
    bio: String,
    created_at: Instant
}

pub struct SignUpAction {}
pub struct DeleteUserAction {}
pub struct ChangeUsernameAction {}
pub struct ChangePasswordAction {}
pub struct ChangeBioAction {}
pub struct ChangeProfilePictureAction {}
pub struct FollowUserAction {}
pub struct UnfollowUserAction {}
pub struct BlockUserAction {}