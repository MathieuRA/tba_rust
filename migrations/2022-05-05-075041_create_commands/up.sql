-- Your SQL goes here
CREATE TABLE commands (
    `id` INT NOT NULL AUTO_INCREMENT,
    `command` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `default_message` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
