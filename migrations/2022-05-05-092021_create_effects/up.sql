-- Your SQL goes here
CREATE TABLE effects (
    `id` INT NOT NULL AUTO_INCREMENT,
    `effect_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `item_id` INT NOT NULL,
    `order` INT NOT NULL,
    `command_id` INT NOT NULL,
    `message` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
