INSERT INTO Account (username, name, image) VALUES ('brawnybrave', '/image-thomas.jpg', 'Thomas Hood');
INSERT INTO Account (username, name, image) VALUES ('hexagon.bestagon', '/image-elijah.jpg', 'Elijah Moss');
INSERT INTO Account (username, name, image) VALUES ('annev1990', '/image-anne.jpg', 'Anne Valentine');
INSERT INTO Account (username, name, image) VALUES ('warlikeduke', '/image-javier.jpg', 'Javier Pollard');
INSERT INTO Account (username, name, image) VALUES ('peppersprime32', '/image-roxanne.jpg', 'Roxanne Travis');
INSERT INTO Account (username, name, image) VALUES ('velvetround', '/image-zena.jpg', 'Zena Kelley');
INSERT INTO Account (username, name, image) VALUES ('countryspirit', '/image-jackson.jpg', 'Jackson Barker');
INSERT INTO Account (username, name, image) VALUES ('soccerviewer8', '/image-george.jpg', 'George Partridge');
INSERT INTO Account (username, name, image) VALUES ('voyager.344', '/image-ryan.jpg', 'Ryan Welles');
INSERT INTO Account (username, name, image) VALUES ('upbeat1811', '/image-suzanne.jpg', 'Suzanne Chang');
INSERT INTO Account (username, name, image) VALUES ('arlen_the_marlin', '/image-victoria.jpg', 'Victoria Mejia');
INSERT INTO Account (username, name, image) VALUES ('hummingbird1', '/image-james.jpg', 'James Skinner');

INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (1, 'Add tags for solutions', 'enhancement', 112, false, 'suggestion', 'Easier to search for solutions based on a specific stack.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (2, 'Add a dark theme option', 'feature', 99, false, 'suggestion', 'It would help people with light sensitivities and who prefer dark mode.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (3, 'Q&A within the challenge hubs', 'feature', 65, false, 'suggestion', 'Challenge-specific Q&A would make for easy reference.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (4, 'Add image/video upload to feedback', 'enhancement', 51, false, 'suggestion', 'Images and screencasts can enhance comments on solutions.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (5, 'Ability to follow others', 'feature', 42, false, 'suggestion', 'Stay updated on comments and solutions other people post.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (6, 'Preview images not loading', 'bug', 3, false, 'suggestion', 'Challenge preview images are missing when you apply a filter.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (7, 'More comprehensive reports', 'feature', 123, false, 'planned', 'It would be great to see a more detailed breakdown of solutions.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (8, 'Learning paths', 'feature', 28, false, 'planned', 'Sequenced projects for different goals to help people improve.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (9, 'One-click portfolio generation', 'feature', 62, false, 'in-progress', 'Add ability to create professional looking portfolio from profile.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (10, 'Bookmark challenges', 'feature', 31, false, 'in-progress', 'Be able to bookmark challenges to take later on.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (11, 'Animated solution screenshots', 'bug', 9, false, 'in-progress', 'Screenshots of solutions with animations don''t display correctly.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (12, 'More comprehensive reports', 'feature', 123, false, 'planned', 'It would be great to see a more detailed breakdown of solutions.');
INSERT INTO Request (id, title, category, upvotes, upvoted, status, description)
    VALUES (13, 'Add micro-interactions', 'enhancement', 71, false, 'live', 'Small animations at specific points can add delight.');

INSERT INTO Comment (id, content, owner) VALUES (1, 'Awesome idea! Trying to find framework-specific projects within the hubs can be tedious', 'upbeat1811');
INSERT INTO Comment (id, content, owner) VALUES (2, 'Please use fun, color-coded labels to easily identify them at a glance', 'brawnybrave');
INSERT INTO Comment (id, content, owner) VALUES (3, 'Also, please allow styles to be applied based on system preferences', 'hexagon.bestagon');
INSERT INTO Comment (id, content, owner) VALUES (4, 'Second this! I do a lot of late night coding and reading', 'hummingbird1');
INSERT INTO Comment (id, content, owner) VALUES (5, 'Much easier to get answers from devs who can relate, since they''ve either finished the challenge themselves or are in the middle of it', 'soccerviewer8');
INSERT INTO Comment (id, content, owner) VALUES (6, 'Right now, there is no ability to add images while giving feedback which isn''t ideal because I have to use another app to show what I mean', 'warlikeduke');
INSERT INTO Comment (id, content, owner) VALUES (7, 'Yes I''d like to see this as well', 'peppersprime32');
INSERT INTO Comment (id, content, owner) VALUES (8, 'I also want to be notified when devs I follow submit projects on FEM', 'arlen_the_marlin');
INSERT INTO Comment (id, content, owner) VALUES (9, 'I''ve been saving the profile URLs of a few people and I check what they''ve been doing frem time to time', 'countryspirit');
INSERT INTO Comment (id, content, owner) VALUES (10, 'This would be awesome! It would be so helpful to see an overview of my code in a way that makes it easy to spot where things could be improved', 'arlen_the_marlin');
INSERT INTO Comment (id, content, owner) VALUES (11, 'Yeah, this would be really good', 'countryspirit');
INSERT INTO Comment (id, content, owner) VALUES (12, 'Having a path through the challenges that I could follow would be brilliant! Sometimes I''m not sure which challenge would be the best next step to take', 'soccerviewer8');
INSERT INTO Comment (id, content, owner) VALUES (13, 'I haven''t built a portfolio site yet, so this would be really helpful', 'voyager.344');
INSERT INTO Comment (id, content, owner) VALUES (14, 'This would be great! At the moment, I''m just starting challenges in order to save them', 'upbeat1811');
INSERT INTO Comment (id, content, owner) VALUES (15, 'I''d love to see this! It always makes me so happy to see little details like these on websites', 'arlen_the_marlin');
INSERT INTO Comment (id, content, owner) VALUES (16, 'Me too! I''d also love to see celebrations at specific points as well', 'upbeat1811');
INSERT INTO Comment (id, content, owner) VALUES (17, 'Bumping this', 'velvetround');
INSERT INTO Comment (id, content, owner) VALUES (18, 'While waiting for dark mode, there are browser extensions that will also do the job', 'annev1990');
INSERT INTO Comment (id, content, owner) VALUES (19, 'Good point! Using any kind of style extension is great and can be highly customizable, like the ability to change contrast and brightness, for security and privacy reasons', 'voyager.344');

INSERT INTO RequestComment (id_request, id_comment) VALUES (1, 1);
INSERT INTO RequestComment (id_request, id_comment) VALUES (1, 2);
INSERT INTO RequestComment (id_request, id_comment) VALUES (2, 3);
INSERT INTO RequestComment (id_request, id_comment) VALUES (2, 4);
INSERT INTO RequestComment (id_request, id_comment) VALUES (3, 5);
INSERT INTO RequestComment (id_request, id_comment) VALUES (4, 6);
INSERT INTO RequestComment (id_request, id_comment) VALUES (4, 7);
INSERT INTO RequestComment (id_request, id_comment) VALUES (5, 9);
INSERT INTO RequestComment (id_request, id_comment) VALUES (7, 10);
INSERT INTO RequestComment (id_request, id_comment) VALUES (7, 11);
INSERT INTO RequestComment (id_request, id_comment) VALUES (8, 12);
INSERT INTO RequestComment (id_request, id_comment) VALUES (9, 13);
INSERT INTO RequestComment (id_request, id_comment) VALUES (10, 14);
INSERT INTO RequestComment (id_request, id_comment) VALUES (12, 15);

INSERT INTO CommentReply (id_parent, id_reply) VALUES (4, 19);
INSERT INTO CommentReply (id_parent, id_reply) VALUES (4, 18);
INSERT INTO CommentReply (id_parent, id_reply) VALUES (8, 17);
INSERT INTO CommentReply (id_parent, id_reply) VALUES (15, 16);