pub trait Action {}

pub struct LogInAction {}
pub struct LogOutAction {}

pub struct SendMessageAction {}

pub struct PostCommentAction {}
pub struct LikeCommentAction {}

fn run_action<T: Action>(action: T) {}