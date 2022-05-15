-- sea_demo.cake definition


CREATE TABLE `cake` (
  `id` int PRIMARY KEY ,
  `name` varchar(255) NOT NULL
);


-- sea_demo.fruit definition

CREATE TABLE `fruit` (
  `id` int PRIMARY KEY ,
  `name` varchar(255) NOT NULL,
  `cake_id` int DEFAULT NULL
);
