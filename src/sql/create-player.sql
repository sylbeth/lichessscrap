DROP TABLE IF EXISTS Player;
CREATE TABLE IF NOT EXISTS Player (
PlayerId INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
Name NCHAR(31) NOT NULL,
UNIQUE (Name)
);
