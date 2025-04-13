DROP database IF exists `lichess`;
CREATE DATABASE IF NOT EXISTS `lichess` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci;
USE `lichess`;

CREATE TABLE IF NOT EXISTS `RuleSet` (
    `RuleSetId` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `Name` NCHAR(31) NOT NULL,
    `URLId` CHAR(8),
    `Kind` ENUM('Arena', 'Swiss', 'Game') NOT NULL,
    `TimeControl` ENUM('SuperBullet','Bullet', 'Blitz', 'Rapid', 'Classical') NOT NULL,
    CONSTRAINT UC_RuleSet UNIQUE (`Name`, `URLId`)
);

CREATE TABLE IF NOT EXISTS `Player` (
    `PlayerId` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    Name NCHAR(31) NOT NULL
);

CREATE TABLE IF NOT EXISTS `Opening` (
  `OpeningId` INT NOT NULL AUTO_INCREMENT,
  `Name` NCHAR(45) NOT NULL,
  `EcoLetter` ENUM('A', 'B', 'C', 'D', 'E', 'NULL') NOT NULL DEFAULT 'NULL',
  `EcoNumber` TINYINT(2) UNSIGNED ZEROFILL NOT NULL DEFAULT 0,
  PRIMARY KEY (`OpeningId`),
  UNIQUE INDEX `OpeningId_UNIQUE` (`openingId` ASC) VISIBLE,
  UNIQUE INDEX `Name_UNIQUE` (`Name` ASC) VISIBLE
);

CREATE TABLE IF NOT EXISTS FinalConfiguration (
    FCId INT PRIMARY KEY AUTO_INCREMENT,
    FEN NCHAR(92) NOT NULL,
    EndPieces INT UNSIGNED NOT NULL,
    UNIQUE (FEN)
);



CREATE TABLE IF NOT EXISTS `Game` (
    `GameId` INT,
    `RuleSetId` INT,
    `OpeningId` INT NULL,
    `FCId` INT,
    `White` INT NULL,
    `WhiteElo` SMALLINT UNSIGNED NULL,
    `WhiteTitle` ENUM('BOT', 'LM', 'GM', 'IM', 'FM', 'CM', 'NM', 'WGM', 'WIM', 'WFM', 'WCM', 'WNM') NULL,
    `Black` INT NULL,
    `BlackElo` SMALLINT UNSIGNED NULL,
    `BlackTitle` ENUM('BOT', 'LM', 'GM', 'IM', 'FM', 'CM', 'NM', 'WGM', 'WIM', 'WFM', 'WCM', 'WNM') NULL,
    `StartTime` SMALLINT UNSIGNED NULL,
    `Increment` TINYINT UNSIGNED NULL,
    `Result` TINYINT NULL,
    `Termination` TINYINT,
    `DateTime` DATETIME,
    `HasClock` TINYINT(1),
    `HasEvaluations` TINYINT(1),
    PRIMARY KEY (GameId),
    FOREIGN KEY (RuleSetId) REFERENCES RuleSet(RuleSetId),
    FOREIGN KEY (OpeningId) REFERENCES Opening(OpeningId),
    FOREIGN KEY (White) REFERENCES Player(PlayerId),
    FOREIGN KEY (Black) REFERENCES Player(PlayerId),
    FOREIGN KEY (FCId) REFERENCES FinalConfiguration(FCId)
);


CREATE TABLE IF NOT EXISTS `Move` (
    `GameId` INT NOT NULL,
    `Num` SMALLINT UNSIGNED NOT NULL,
    `Descriptor` INT NOT NULL,
    `EvalFloat` FLOAT NULL,
    `EvalInt` INT NULL,
    `Clock` TIME NULL,
    PRIMARY KEY (`GameId`, `Num`),
    FOREIGN KEY (`GameId`) REFERENCES Game(`GameId`)
);