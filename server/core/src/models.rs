pub struct VideoLike {
    user_id: UserId,
    video_id: VideoId,
    created_at: Instant
}

pub type CommentId = u64;

pub struct Comment {
    id: CommentId,
    text: String,
    parent_comment_id: CommentId,
    created_at: Instant
}

pub struct CommentLike {
    user_id: UserId,
    comment_id: CommentId,
    created_at: Instant
}

pub type MessageId = u64;

pub struct Message {
    id: MessageId,
    text: String
}

pub type SongId = u64;

pub struct Song {
    id: SongId,
    song_file_url: String,
    name: String
}

pub struct Follower {
    user_id: UserId,
    followed_user_id: UserId
}

pub struct UserLike {
    user_id: UserId,
    liked_user_id: UserId,
    created_at: Instant
}