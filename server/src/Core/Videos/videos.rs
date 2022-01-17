type VideoId = u64;

struct Video {
    id: VideoId,
    video_file_url: String,
    description: String,
    song: Song,
    likes: Vec<Like>,
    comments: Vec<Comment>,
    comment_replied_to: Maybe<Comment>,
    uploaded_at: Instant
}

struct UploadVideoAction {}
struct ChangeVideoDescriptionAction {}
struct LikeVideoAction {}