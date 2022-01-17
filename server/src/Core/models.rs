struct VideoLike {
    user_id: UserId,
    video_id: VideoId,
    created_at: Instant
}

type CommentId = u64;

struct Comment {
    id: CommentId,
    text: String,
    parent_comment_id: CommentId,
    created_at: Instant
}

struct CommentLike {
    user_id: UserId,
    comment_id: CommentId,
    created_at: Instant
}

type MessageId = u64;

struct Message {
    id: MessageId,
    text: String
}

type SongId = u64;

struct Song {
    id: SongId,
    song_file_url: String,
    name: String
}

struct Follower {
    user_id: UserId,
    followed_user_id: UserId
}

struct UserLike {
    user_id: UserId,
    liked_user_id: UserId,
    created_at: Instant
}