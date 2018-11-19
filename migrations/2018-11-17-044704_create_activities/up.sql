CREATE VIEW activities
  AS WITH raw_activities as (SELECT shares.actor      as actor,
                                    shares.object     as object,
                                    shares.target     as target,
                                    shares.created_at as created_at,
                                    'share'           as activity_type
                             FROM shares
                             UNION
                             SELECT likes.actor      as actor,
                                    likes.object     as object,
                                    likes.target     as target,
                                    likes.created_at as created_at,
                                    'like'           as activity_type
                             FROM likes
                             UNION
                             SELECT posts.actor      as actor,
                                    posts.object     as object,
                                    NULL             as target,
                                    posts.created_at as created_at,
                                    'post'           as activity_type
                             FROM posts)
  SELECT actor_users.name             as actor,
         raw_activities.activity_type as verb,
         raw_activities.object        as object,
         target_users.name            as target,
         raw_activities.created_at    as created_at
  FROM raw_activities
         LEFT JOIN users as target_users on target_users.id = raw_activities.target
         JOIN users as actor_users on actor_users.id = raw_activities.actor;
