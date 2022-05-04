-- Your SQL goes here
CREATE TABLE directions (
    `id` INT NOT NULL AUTO_INCREMENT,
    `command` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB;