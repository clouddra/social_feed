DELETE FROM users;
INSERT INTO users(name) VALUES('peter');
INSERT INTO users(name) VALUES('paul');
INSERT INTO users(name) VALUES('ivan');
INSERT INTO users(name) VALUES('eric');
INSERT INTO users(name) VALUES('niko');

DELETE FROM follows;
-- peter -> paul, ivan, niko
-- paul -> ivan, eric
INSERT INTO follows VALUES(1, 2);
INSERT INTO follows VALUES(1, 5);
INSERT INTO follows VALUES(2, 3);
INSERT INTO follows VALUES(2, 4);
INSERT INTO follows VALUES(1, 3);
INSERT INTO follows VALUES(5, 4);

DELETE FROM posts;
INSERT INTO posts(actor, object, created_at) VALUES(1, 'post:peter:1', '2018-11-01 00:00:00');
INSERT INTO posts(actor, object, created_at) VALUES(1, 'post:peter:2', '2018-11-02 00:00:00');
INSERT INTO posts(actor, object, created_at) VALUES(1, 'post:peter:3', '2018-11-03 00:00:00');
INSERT INTO posts(actor, object, created_at) VALUES(1, 'post:peter:4', '2018-11-04 00:00:00');
INSERT INTO posts(actor, object, created_at) VALUES(1, 'post:peter:5', '2018-11-05 00:00:00');

INSERT INTO posts(actor, object, created_at) VALUES(2, 'post:paul:1', '2018-11-01 00:00:01');
INSERT INTO posts(actor, object, created_at) VALUES(2, 'post:paul:2', '2018-11-02 00:00:01');
INSERT INTO posts(actor, object, created_at) VALUES(2, 'post:paul:3', '2018-11-03 00:00:01');
INSERT INTO posts(actor, object, created_at) VALUES(2, 'post:paul:4', '2018-11-04 00:00:01');
INSERT INTO posts(actor, object, created_at) VALUES(2, 'post:paul:5', '2018-11-05 00:00:01');

INSERT INTO posts(actor, object, created_at) VALUES(3, 'post:ivan:1', '2018-11-01 00:00:02');
INSERT INTO posts(actor, object, created_at) VALUES(3, 'post:ivan:2', '2018-11-02 00:00:02');
INSERT INTO posts(actor, object, created_at) VALUES(3, 'post:ivan:3', '2018-11-03 00:00:02');
INSERT INTO posts(actor, object, created_at) VALUES(3, 'post:ivan:4', '2018-11-04 00:00:02');
INSERT INTO posts(actor, object, created_at) VALUES(3, 'post:ivan:5', '2018-11-05 00:00:02');

INSERT INTO posts(actor, object, created_at) VALUES(4, 'post:eric:1', '2018-11-01 00:00:03');
INSERT INTO posts(actor, object, created_at) VALUES(4, 'post:eric:2', '2018-11-02 00:00:03');
INSERT INTO posts(actor, object, created_at) VALUES(4, 'post:eric:3', '2018-11-03 00:00:03');
INSERT INTO posts(actor, object, created_at) VALUES(4, 'post:eric:4', '2018-11-04 00:00:03');
INSERT INTO posts(actor, object, created_at) VALUES(4, 'post:eric:5', '2018-11-05 00:00:03');

INSERT INTO posts(actor, object, created_at) VALUES(5, 'post:niko:1', '2018-11-01 00:00:04');
INSERT INTO posts(actor, object, created_at) VALUES(5, 'post:niko:2', '2018-11-02 00:00:04');
INSERT INTO posts(actor, object, created_at) VALUES(5, 'post:niko:3', '2018-11-03 00:00:04');
INSERT INTO posts(actor, object, created_at) VALUES(5, 'post:niko:4', '2018-11-04 00:00:04');
INSERT INTO posts(actor, object, created_at) VALUES(5, 'post:niko:5', '2018-11-05 00:00:04');

DELETE FROM likes;
INSERT INTO likes(actor, object, target, created_at) VALUES(1, 'post:niko:1', 5, '2018-11-06 01:00:00');
INSERT INTO likes(actor, object, target, created_at) VALUES(1, 'post:ivan:2', 3, '2018-11-06 02:00:00');
INSERT INTO likes(actor, object, target, created_at) VALUES(1, 'post:paul:2', 2, '2018-11-06 03:00:00');
INSERT INTO likes(actor, object, target, created_at) VALUES(1, 'post:paul:3', 2, '2018-11-06 04:00:00');

DELETE FROM shares;
INSERT INTO shares(actor, object, target, created_at) VALUES(2, 'post:niko:1', 5, '2018-11-06 00:01:00');
INSERT INTO shares(actor, object, target, created_at) VALUES(2, 'post:ivan:2', 3, '2018-11-06 00:02:00');
INSERT INTO shares(actor, object, target, created_at) VALUES(2, 'post:paul:2', 2, '2018-11-06 00:03:00');
INSERT INTO shares(actor, object, target, created_at) VALUES(2, 'post:paul:3', 2, '2018-11-06 00:04:00');
