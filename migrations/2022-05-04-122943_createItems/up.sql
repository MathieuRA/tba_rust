-- Your SQL goes here
CREATE TABLE items (
    `id` INT NOT NULL AUTO_INCREMENT,
    `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `room_id` INT NOT NULL,
    PRIMARY KEY (`id`),
    KEY `room_id` (`room_id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;