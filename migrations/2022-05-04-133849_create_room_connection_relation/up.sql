-- Your SQL goes here
ALTER TABLE room_connections
  ADD CONSTRAINT `from_room_id` FOREIGN KEY (`from_room_id`) REFERENCES `rooms` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT,
  ADD CONSTRAINT `to_room_id` FOREIGN KEY (`to_room_id`) REFERENCES `rooms` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT,
  ADD CONSTRAINT `direction_id` FOREIGN KEY (`direction_id`) REFERENCES `directions` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT;