INSERT INTO Account (username, name, image) VALUES
    ('brawnybrave', '/image-thomas.jpg', 'Thomas Hood'),
    ('hexagon.bestagon', '/image-elijah.jpg', 'Elijah Moss'),
    ('annev1990', '/image-anne.jpg', 'Anne Valentine'),
    ('warlikeduke', '/image-javier.jpg', 'Javier Pollard'),
    ('peppersprime32', '/image-roxanne.jpg', 'Roxanne Travis'),
    ('velvetround', '/image-zena.jpg', 'Zena Kelley'),
    ('countryspirit', '/image-jackson.jpg', 'Jackson Barker'),
    ('soccerviewer8', '/image-george.jpg', 'George Partridge'),
    ('voyager.344', '/image-ryan.jpg', 'Ryan Welles'),
    ('upbeat1811', '/image-suzanne.jpg', 'Suzanne Chang'),
    ('arlen_the_marlin', '/image-victoria.jpg', 'Victoria Mejia'),
    ('hummingbird1', '/image-james.jpg', 'James Skinner');

INSERT INTO Request (title, category, upvotes, upvoted, status, description) VALUES
    ('Add tags for solutions', 'enhancement', 112, false, 'suggestion', 'Easier to search for solutions based on a specific stack.'),
    ('Add a dark theme option', 'feature', 99, false, 'suggestion', 'It would help people with light sensitivities and who prefer dark mode.'),
    ('Q&A within the challenge hubs', 'feature', 65, false, 'suggestion', 'Challenge-specific Q&A would make for easy reference.'),
    ('Add image/video upload to feedback', 'enhancement', 51, false, 'suggestion', 'Images and screencasts can enhance comments on solutions.'),
    ('Ability to follow others', 'feature', 42, false, 'suggestion', 'Stay updated on comments and solutions other people post.'),
    ('Preview images not loading', 'bug', 3, false, 'suggestion', 'Challenge preview images are missing when you apply a filter.'),
    ('More comprehensive reports', 'feature', 123, false, 'planned', 'It would be great to see a more detailed breakdown of solutions.'),
    ('Learning paths', 'feature', 28, false, 'planned', 'Sequenced projects for different goals to help people improve.'),
    ('One-click portfolio generation', 'feature', 62, false, 'in-progress', 'Add ability to create professional looking portfolio from profile.'),
    ('Bookmark challenges', 'feature', 31, false, 'in-progress', 'Be able to bookmark challenges to take later on.'),
    ('Animated solution screenshots', 'bug', 9, false, 'in-progress', 'Screenshots of solutions with animations don''t display correctly.'),
    ('More comprehensive reports', 'feature', 123, false, 'planned', 'It would be great to see a more detailed breakdown of solutions.'),
    ('Add micro-interactions', 'enhancement', 71, false, 'live', 'Small animations at specific points can add delight.');

INSERT INTO Comment (id_request, id_parent, content, owner) VALUES
    (1, NULL, 'Awesome idea! Trying to find framework-specific projects within the hubs can be tedious', 'upbeat1811'),
    (1, NULL,'Please use fun, color-coded labels to easily identify them at a glance', 'brawnybrave'),
    (2, NULL,'Also, please allow styles to be applied based on system preferences', 'hexagon.bestagon'),
    (2, NULL, 'Second this! I do a lot of late night coding and reading', 'hummingbird1'),
    (3, NULL, 'Much easier to get answers from devs who can relate, since they''ve either finished the challenge themselves or are in the middle of it', 'soccerviewer8'),
    (4, NULL, 'Right now, there is no ability to add images while giving feedback which isn''t ideal because I have to use another app to show what I mean', 'warlikeduke'),
    (4, NULL, 'Yes I''d like to see this as well', 'peppersprime32'),
    (5, NULL, 'I also want to be notified when devs I follow submit projects on FEM', 'arlen_the_marlin'),
    (5, NULL, 'I''ve been saving the profile URLs of a few people and I check what they''ve been doing frem time to time', 'countryspirit'),
    (7, NULL, 'This would be awesome! It would be so helpful to see an overview of my code in a way that makes it easy to spot where things could be improved', 'arlen_the_marlin'),
    (7, NULL, 'Yeah, this would be really good', 'countryspirit'),
    (8, NULL, 'Having a path through the challenges that I could follow would be brilliant! Sometimes I''m not sure which challenge would be the best next step to take', 'soccerviewer8'),
    (9, NULL, 'I haven''t built a portfolio site yet, so this would be really helpful', 'voyager.344'),
    (10, NULL, 'This would be great! At the moment, I''m just starting challenges in order to save them', 'upbeat1811'),
    (12, NULL, 'I''d love to see this! It always makes me so happy to see little details like these on websites', 'arlen_the_marlin'),
    (12, 15, 'Me too! I''d also love to see celebrations at specific points as well', 'upbeat1811'),
    (5, 8, 'Bumping this', 'velvetround'),
    (2, 4, 'While waiting for dark mode, there are browser extensions that will also do the job', 'annev1990'),
    (2, 4, 'Good point! Using any kind of style extension is great and can be highly customizable, like the ability to change contrast and brightness, for security and privacy reasons', 'voyager.344');
