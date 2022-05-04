-- Your SQL goes here
ALTER TABLE `items` ADD CONSTRAINT `relation_item_room` FOREIGN KEY (`room_id`) REFERENCES `rooms`(`id`) ON DELETE CASCADE ON UPDATE RESTRICT;