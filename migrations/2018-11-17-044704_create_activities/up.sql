CREATE VIEW activities AS
SELECT *, 'share' as activity_type FROM shares
UNION
SELECT *, 'like' as activity_type FROM likes
UNION
SELECT posts.id as id, posts.actor as actor, posts.object as object, NULL as target, posts.created_at as created_at, 'post' as activity_type FROM posts;
