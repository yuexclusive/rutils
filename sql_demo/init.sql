-- sea_demo.cake definition

drop table if exists `cake`;

CREATE TABLE `cake` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

INSERT INTO cake (id,name) VALUES
	 (1,'New York Cheese'),
	 (2,'Chocolate Forest')

drop table if exists `fruit`;

-- sea_demo.fruit definition

CREATE TABLE `fruit` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `cake_id` int DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


INSERT INTO sea_demo.fruit (name,cake_id) VALUES
	 ('Blueberry',1),
	 ('Rasberry',1),
	 ('Strawberry',2),
	 ('Apple',NULL),
	 ('Banana',NULL),
	 ('Cherry',NULL),
	 ('Lemon',NULL),
	 ('Orange',NULL),
	 ('Pineapple',NULL),
	 ('Sweet pear',NULL),
	 ('Pineapple',NULL),
	 ('test',NULL);