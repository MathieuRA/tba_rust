-- Your SQL goes here
CREATE TABLE room_connections (
    `id` INT NOT NULL AUTO_INCREMENT,
    `from_room_id` INT UNSIGNED NOT NULL,
    `to_room_id` INT UNSIGNED NOT NULL,
    `direction_id` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `from_room_id_direction_id` (`from_room_id`,`direction_id`),
  KEY `to_room_id` (`to_room_id`),
  KEY `direction_id` (`direction_id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
