-- DATABASE SCHEMA FOR RHODEV ISLAND LOGISTIC
-- Jangan disentuh ya adik-adik, ini schema database untuk Rhodev Island Logistic. Jangan diubah-ubah, nanti error.
-- Cukup pakai file sqlnya buat di phpmyadmin.

BEGIN;

CREATE DATABASE IF NOT EXISTS `rhodev_island_logistic`;

USE `rhodev_island_logistic`;

-- Cartesian mini Terra: x = 0..1000 (east), y = 0..800 (north); origin at southwest.
-- Seeded locations keep a 50-unit outer margin; all coordinates are simulation values.
-- Regional schematic and frame: terra-map-layout.json; reference: https://arknights.wiki.gg/wiki/Geography
-- Operator deployment is randomized with seed 20261102 near suitable warehouse hubs.
-- Initial operator availability is TRUE (on duty).

CREATE TABLE IF NOT EXISTS `region` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS `race` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS `role` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS `operator` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `fullname` VARCHAR(255) NOT NULL,
  `codename` VARCHAR(255) NOT NULL,
  `age` INT NOT NULL,
  `gender` VARCHAR(10) NOT NULL,
  `place_of_birth` INT NOT NULL,
  `date_of_birth` DATE NOT NULL,
  `race_id` INT NOT NULL,
  `height` FLOAT NOT NULL,
  `role_id` INT NOT NULL,
  `region_specialty_id` INT DEFAULT NULL,
  `completed_job_count` INT NOT NULL DEFAULT 0 CHECK (`completed_job_count` >= 0),
  `x_position` FLOAT NOT NULL,
  `y_position` FLOAT NOT NULL,
  `availability` BOOLEAN NOT NULL,

  FOREIGN KEY (`role_id`) REFERENCES `role`(`id`),
  FOREIGN KEY (`race_id`) REFERENCES `race`(`id`),
  FOREIGN KEY (`place_of_birth`) REFERENCES `region`(`id`),
  FOREIGN KEY (`region_specialty_id`) REFERENCES `region`(`id`)
);

CREATE TABLE IF NOT EXISTS `medicine_type` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS `manufacturer` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL,
  `region_id` INT NOT NULL,
  `x_position` FLOAT NOT NULL,
  `y_position` FLOAT NOT NULL,

  FOREIGN KEY (`region_id`) REFERENCES `region`(`id`)
);

CREATE TABLE IF NOT EXISTS `warehouse` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL,
  `region_id` INT NOT NULL,
  `capacity` INT NOT NULL DEFAULT 0,
  `x_position` FLOAT NOT NULL,
  `y_position` FLOAT NOT NULL,

  FOREIGN KEY (`region_id`) REFERENCES `region`(`id`)
);

CREATE TABLE IF NOT EXISTS `medicine` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL,
  `medicine_type_id` INT NOT NULL,
  `dosage_form` VARCHAR(255) NOT NULL,
  `manufacturer_id` INT NOT NULL,
  `description` TEXT,

  FOREIGN KEY (`medicine_type_id`) REFERENCES `medicine_type`(`id`),
  FOREIGN KEY (`manufacturer_id`) REFERENCES `manufacturer`(`id`)
);

CREATE TABLE IF NOT EXISTS `order` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `warehouse_id` INT NOT NULL,

  FOREIGN KEY (`warehouse_id`) REFERENCES `warehouse`(`id`)
);

CREATE TABLE IF NOT EXISTS `order_status` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `status` VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS `order_item` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `order_id` INT NOT NULL,
  `medicine_id` INT NOT NULL,
  `quantity` INT NOT NULL,
  `status` INT NOT NULL,
  `courier_id` INT DEFAULT NULL,
  `pickup_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `delivery_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (`order_id`) REFERENCES `order`(`id`),
  FOREIGN KEY (`medicine_id`) REFERENCES `medicine`(`id`),
  FOREIGN KEY (`status`) REFERENCES `order_status`(`id`),
  FOREIGN KEY (`courier_id`) REFERENCES `operator`(`id`)
);

CREATE TABLE IF NOT EXISTS `activity_log` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `description` TEXT NOT NULL,
  `timestamp` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS `warehouse_inventory` (
  `warehouse_id` INT NOT NULL,
  `medicine_id` INT NOT NULL,
  `quantity` INT NOT NULL DEFAULT 0 CHECK (`quantity` >= 0),
  `reorder_threshold` INT NOT NULL DEFAULT 0 CHECK (`reorder_threshold` >= 0),

  PRIMARY KEY (`warehouse_id`, `medicine_id`),
  FOREIGN KEY (`warehouse_id`) REFERENCES `warehouse`(`id`),
  FOREIGN KEY (`medicine_id`) REFERENCES `medicine`(`id`)
);

CREATE TABLE IF NOT EXISTS `event_type` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL,
  `risk_level` INT NOT NULL CHECK (`risk_level` BETWEEN 1 AND 5)
);

CREATE TABLE IF NOT EXISTS `event_log` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `description` VARCHAR(255) NOT NULL,
  `event_type_id` INT NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (`event_type_id`) REFERENCES `event_type`(`id`)
);

COMMIT;

USE `rhodev_island_logistic`;

BEGIN;

INSERT INTO `region` (`id`, `name`) VALUES
(1, 'Aegir'),
(2, 'Bolivar'),
(3, 'Columbia'),
(4, 'Durin'),
(5, 'Higashi'),
(6, 'Iberia'),
(7, 'Kazdel'),
(8, 'Kazimierz'),
(9, 'Kjerag'),
(10, 'Laterano'),
(11, 'Leithanien'),
(12, 'Minos'),
(13, 'Rim Billiton'),
(14, 'Sami'),
(15, 'Sargon'),
(16, 'Siesta'),
(17, 'Siracusa'),
(18, 'Ursus'),
(19, 'Victoria'),
(20, 'Yan'),
(21, 'Tara');

COMMIT;

USE `rhodev_island_logistic`;

BEGIN;

INSERT INTO `race` (`id`, `name`) VALUES
(1, 'Aegir'),
(2, 'Anasa'),
(3, 'Anaty'),
(4, 'Anura'),
(5, 'Archosauria'),
(6, 'Aslan'),
(7, 'Caprinae'),
(8, 'Cautus'),
(9, 'Cerato'),
(10, 'Chimera'),
(11, 'Draco'),
(12, 'Durin'),
(13, 'Elafia'),
(14, 'Elf'),
(15, 'Feline'),
(16, 'Forte'),
(17, 'Itra'),
(18, 'Kukulkan'),
(19, 'Kuranta'),
(20, 'Kylin'),
(21, 'Liberi'),
(22, 'Lung'),
(23, 'Lupo'),
(24, 'Manticore'),
(25, 'Oni'),
(26, 'Perro'),
(27, 'Petram'),
(28, 'Phidia'),
(29, 'Pilosa'),
(30, 'Reproba'),
(31, 'Sankta'),
(32, 'Sarkaz'),
(33, 'Savra'),
(34, 'Ursus'),
(35, 'Vouivre'),
(36, 'Vulpo'),
(37, 'Zalak');

COMMIT;


USE `rhodev_island_logistic`;

BEGIN;

INSERT INTO `role` (`id`, `name`) VALUES
(1, 'Dispatcher'),
(2, 'Courier');

-- Each operator row also carries the job profile (specialty, position, availability).
-- Regional specialties are gameplay inferences from verified origins or work experience.
-- Basic-profile birthplace reference (story-based exceptions have individual citations below):
-- https://github.com/Kengxxiao/ArknightsGameData_YoStar/blob/main/en_US/gamedata/excel/handbook_info_table.json
INSERT INTO `operator`
(`fullname`, `codename`, `age`, `gender`, `place_of_birth`, `date_of_birth`, `race_id`, `height`, `role_id`, `region_specialty_id`, `completed_job_count`, `x_position`, `y_position`, `availability`)
VALUES
-- Amiya; race: Chimera; birth region: Rim Billiton; https://arknights.wiki.gg/wiki/Amiya
-- Amiya: Rim Billiton; documented birthplace.
('Amiya', 'Amiya', 18, 'Female', 13, '2007-12-23', 10, 142, 1, 13, 0, 717.85, 268.44, TRUE),
-- Blaze; race: Feline; birth region: Victoria; https://arknights.wiki.gg/wiki/Blaze/Story
-- Blaze: Victoria; documented birthplace.
('Gu Zhuhuang', 'Blaze', 28, 'Female', 19, '1998-06-15', 15, 172, 2, 19, 0, 464.08, 420.28, TRUE),
-- Logos; race: Sarkaz; birth region: Kazdel; https://arknights.wiki.gg/wiki/Logos/Story
-- Logos: Kazdel; documented birthplace.
('Aefanyl', 'Logos', 30, 'Male', 7, '1996-09-05', 32, 178, 2, 7, 0, 712.69, 472.22, TRUE),
-- Rosmontis; race: Feline; birth region: Columbia; https://arknights.wiki.gg/wiki/Rosmontis/Story
-- Rosmontis: Columbia; documented birthplace.
('Narcissa', 'Rosmontis', 17, 'Female', 3, '2009-07-06', 15, 142, 2, 3, 0, 259.19, 487.42, TRUE),
-- Ascalon; race: Sarkaz; birth region: Kazdel; https://arknights.wiki.gg/wiki/Ascalon
-- Ascalon: Kazdel; documented birthplace.
('Ascalon', 'Ascalon', 42, 'Female', 7, '1984-10-04', 32, 175, 2, 7, 0, 681.18, 481.26, TRUE),
-- Dobermann; race: Perro; birth region: Bolivar; https://arknights.wiki.gg/wiki/Dobermann
-- Dobermann: Bolivar; documented birthplace and military background.
('Dobermann', 'Dobermann', 32, 'Female', 2, '1994-05-27', 26, 163, 2, 2, 0, 105.66, 416.38, TRUE),
-- Ch'en; race: Lung; Rhodes Island; birth region: Yan; https://arknights.wiki.gg/wiki/Ch%27en/Story
-- Chen: Lungmen background, represented by Yan in the region catalogue.
('Chen Hui-chieh', 'Chen', 28, 'Female', 20, '1998-07-07', 22, 168, 2, 20, 0, 843.38, 444.49, TRUE),
-- Gavial; race: Archosauria; birth region: Sargon; https://arknights.wiki.gg/wiki/Gavial
-- Gavial: Acahualla, Sargon; upbringing and medical work.
-- https://arknights.wiki.gg/wiki/Gavial/Story
('Gavial', 'Gavial', 26, 'Female', 15, '2000-05-01', 5, 163, 2, 15, 0, 193.98, 205.83, TRUE),
-- Warfarin; race: Sarkaz; birth region: Kazdel; https://arknights.wiki.gg/wiki/Warfarin
-- Warfarin: Kazdel; documented birthplace.
('Warfarin', 'Warfarin', 210, 'Female', 7, '1816-06-27', 32, 157, 2, 7, 0, 747.27, 472.33, TRUE),
-- Perfumer; race: Vulpo; birth region: Minos; https://arknights.wiki.gg/wiki/Perfumer/Story
-- Perfumer: Minos; documented birthplace.
('Lena Athopoulou', 'Perfumer', 25, 'Female', 12, '2001-01-30', 36, 158, 2, 12, 0, 333.07, 329.38, TRUE),
-- Kal'tsit; race: Feline; current Esperanta identity; birth-region fixture: Victoria; https://arknights.wiki.gg/wiki/Kal%27tsit_-_Esperanta
-- Kaltsit: No single regional specialty selected; the simulated Victoria birthplace is not used.
('Kaltsit', 'Kaltsit', 42, 'Female', 19, '1984-01-23', 15, 169, 1, NULL, 0, 452.26, 424.31, TRUE),
-- Closure; race: Sarkaz; Engineering and Procurement head; birth region: Kazdel; https://arknights.wiki.gg/wiki/Closure/Story
-- Closure: Kazdel; engineering and infrastructure background.
-- https://arknights.wiki.gg/wiki/Closure/Story
('Ada Church', 'Closure', 120, 'Female', 7, '1905-12-10', 32, 158, 2, 7, 0, 763.33, 491.21, TRUE),
-- Raidian; race: Liberi; Rhodes Island Elite Ops; birth region: Ursus; https://arknights.wiki.gg/wiki/Raidian/Story
-- Raidian: Ursus; education and former military service.
-- https://arknights.wiki.gg/wiki/Raidian/Story
('Serafina Ivanovna Petrova', 'Raidian', 35, 'Female', 18, '1991-02-21', 21, 159, 2, 18, 0, 662.55, 587.09, TRUE),
-- Paprika; race: Sarkaz; birth region: Columbia; https://arknights.wiki.gg/wiki/Paprika
-- Paprika: Columbia; documented birthplace.
('Paprika', 'Paprika', 12, 'Female', 3, '2014-03-13', 32, 155, 2, 3, 0, 246.76, 528.42, TRUE),
-- Mechanist; race: Elafia; Rhodes Island Elite Ops; birth region: Columbia; https://arknights.wiki.gg/wiki/Mechanist/Story
-- Mechanist: Columbia; field operations and knowledge of Trimounts.
-- https://arknights.wiki.gg/wiki/DV-1/Story
('Mechanist', 'Mechanist', 35, 'Male', 3, '1991-07-16', 13, 180, 2, 3, 0, 217.14, 478.45, TRUE),
-- Touch; race: Caprinae; Rhodes Island Elite Ops; birth region: Leithanien; https://arknights.wiki.gg/wiki/Touch/Story
-- Touch: Leithanien; documented background.
-- https://arknights.fandom.com/wiki/Touch/Story
('Touch', 'Touch', 42, 'Female', 11, '1983-11-12', 7, 152, 2, 11, 0, 571.43, 460.40, TRUE),
-- Weedy; race: Aegir; birth region: Iberia; https://arknights.wiki.gg/wiki/Weedy/Story
-- Weedy: Iberia; documented birthplace.
('Penia', 'Weedy', 23, 'Female', 6, '2003-01-23', 1, 155, 2, 6, 0, 536.47, 162.63, TRUE),
-- Sharp; race: Phidia; Rhodes Island Elite Ops; birth region: Columbia; https://arknights.wiki.gg/wiki/Sharp/Story
-- Sharp: Sargon; around twenty years of mercenary experience, rather than his Columbian birthplace.
-- https://arknights.wiki.gg/wiki/Sharp/File
('Sharp', 'Sharp', 46, 'Male', 3, '1979-12-19', 28, 180, 2, 15, 0, 343.39, 165.02, TRUE),
-- Mantra; race: Phidia; Rhodes Island Elite Ops; birth region: Sargon; https://arknights.wiki.gg/wiki/Mantra
-- Mantra: Sargon; former padishah background.
-- https://closure.wiki/operators/mantra
('Mantra', 'Mantra', 95, 'Female', 15, '1931-04-07', 28, 191, 2, 15, 0, 173.78, 190.48, TRUE),
-- Bagpipe; race: Vouivre; retired from Victoria's army and joined Rhodes Island; birth region: Victoria; https://arknights.wiki.gg/wiki/Bagpipe/File
-- Bagpipe: Victoria; birthplace and military service.
('Fiona Young', 'Bagpipe', 25, 'Female', 19, '2001-07-17', 35, 167, 2, 19, 0, 419.01, 352.15, TRUE),
-- Pith; race: Liberi; Rhodes Island Elite Ops; birth region: Victoria; https://arknights.wiki.gg/wiki/Pith/Story
-- Pith: Victoria; military education and service.
-- https://arknights.wiki.gg/wiki/Pith/Story
('Pith', 'Pith', 40, 'Female', 19, '1986-06-14', 21, 165, 2, 19, 0, 462.56, 362.42, TRUE),
-- Misery; race: Sarkaz; Rhodes Island Elite Ops; birth-region fixture: Kazdel; simulated birthday and height; https://arknights.wiki.gg/wiki/Misery/Story
-- Misery: Kazdel; documented field operations and local contacts.
-- https://arknights.wiki.gg/wiki/15-5/Story
('Misery', 'Misery', 45, 'Male', 7, '1981-06-01', 32, 180, 2, 7, 0, 701.83, 462.64, TRUE),
-- Fang; race: Kuranta; birth region: Kazimierz; https://arknights.wiki.gg/wiki/Fang
-- Fang: Kazimierz; documented birthplace.
('Fang', 'Fang', 20, 'Female', 8, '2005-12-02', 19, 158, 2, 8, 0, 429.87, 550.66, TRUE),
-- Kroos; race: Cautus; birth region: Rim Billiton; https://arknights.wiki.gg/wiki/Kroos
-- Kroos: Rim Billiton; documented birthplace.
('Kroos', 'Kroos', 20, 'Female', 13, '2005-11-22', 8, 154, 2, 13, 0, 704.93, 321.19, TRUE),
-- Hibiscus; race: Sarkaz; birth region: Victoria; https://arknights.wiki.gg/wiki/Hibiscus
-- Hibiscus: Victoria; documented birthplace.
('Hibiscus', 'Hibiscus', 21, 'Female', 19, '2005-06-21', 32, 153, 2, 19, 0, 500.88, 432.15, TRUE),
-- Lava; race: Sarkaz; birth region: Victoria; https://arknights.wiki.gg/wiki/Lava
-- Lava: Victoria; documented birthplace.
('Lava', 'Lava', 21, 'Female', 19, '2005-06-22', 32, 154, 2, 19, 0, 459.48, 399.21, TRUE),
-- Melantha; race: Feline; birth region: Victoria; https://arknights.wiki.gg/wiki/Melantha
-- Melantha: Victoria; documented birthplace.
('Melantha', 'Melantha', 19, 'Female', 19, '2007-03-19', 15, 161, 2, 19, 0, 458.18, 377.83, TRUE),
-- Cardigan; race: Perro; birth region: Leithanien; https://arknights.wiki.gg/wiki/Cardigan/Story
-- Cardigan: Leithanien; documented birthplace.
('Merry', 'Cardigan', 19, 'Female', 11, '2007-08-02', 26, 156, 2, 11, 0, 573.93, 487.93, TRUE),
-- Adnachiel; race: Sankta; birth region: Laterano; https://arknights.wiki.gg/wiki/Adnachiel
-- Adnachiel: Laterano; documented birthplace.
('Adnachiel', 'Adnachiel', 20, 'Male', 10, '2006-05-30', 31, 171, 2, 10, 0, 590.21, 252.77, TRUE),
-- Ansel; race: Cautus; birth region: Rim Billiton; https://arknights.wiki.gg/wiki/Ansel
-- Ansel: Rim Billiton; documented birthplace.
('Ansel', 'Ansel', 21, 'Male', 13, '2005-01-31', 8, 163, 2, 13, 0, 678.77, 312.91, TRUE),
-- Ceobe; race: Perro; Rhodes Island; recorded profile race: Perro; birth region: Bolivar; https://arknights.wiki.gg/wiki/Ceobe
-- Ceobe: Bolivar; documented birthplace.
('Ceobe', 'Ceobe', 19, 'Female', 2, '2007-03-03', 26, 153, 2, 2, 0, 119.26, 395.66, TRUE),
-- Orchid; race: Liberi; birth region: Columbia; https://arknights.wiki.gg/wiki/Orchid
-- Orchid: Columbia; documented birthplace.
('Orchid', 'Orchid', 30, 'Female', 3, '1996-01-15', 21, 164, 2, 3, 0, 234.51, 558.94, TRUE),
-- Elysium; race: Liberi; Rhodes Island Special Operations; birth region: Iberia; https://arknights.wiki.gg/wiki/Elysium/Story
-- Elysium: Iberia; documented birthplace.
('Elysium', 'Elysium', 25, 'Male', 6, '2001-05-08', 21, 187, 2, 6, 0, 524.43, 164.90, TRUE),
-- Meteorite; race: Sarkaz; Rhodes Island; birth region: Kazdel; https://arknights.wiki.gg/wiki/Meteorite/File
-- Meteorite: Kazdel; documented birthplace.
('Meteorite', 'Meteorite', 28, 'Female', 7, '1998-08-30', 32, 170, 2, 7, 0, 719.13, 491.44, TRUE),
-- Stormeye; race: Sarkaz; Rhodes Island Elite Ops; birth region: Kazdel; https://arknights.wiki.gg/wiki/Stormeye/Story
-- Stormeye: Kazdel; family background and military experience.
-- https://arknights.wiki.gg/wiki/Stormeye/File
('Stormeye', 'Stormeye', 40, 'Male', 7, '1986-02-12', 32, 180, 2, 7, 0, 751.83, 484.18, TRUE),
-- Popukar; race: Cautus; birth region: Rim Billiton; https://arknights.wiki.gg/wiki/Popukar
-- Popukar: Rim Billiton; documented birthplace.
('Popukar', 'Popukar', 12, 'Female', 13, '2014-07-02', 8, 144, 2, 13, 0, 726.42, 286.31, TRUE),
-- Yato; race: Oni; birth region: Higashi; https://arknights.wiki.gg/wiki/Yato
-- Yato: Higashi; documented birthplace.
('Yato', 'Yato', 29, 'Female', 5, '1997-05-14', 25, 161, 2, 5, 0, 853.86, 597.92, TRUE),
-- Rangers; race: Savra; birth region: Sargon; https://arknights.wiki.gg/wiki/Rangers
-- Rangers: Sargon; former ranger experience.
-- https://arknights.wiki.gg/wiki/Rangers/Story
('Rangers', 'Rangers', 64, 'Male', 15, '1962-07-22', 33, 179, 2, 15, 0, 206.66, 184.43, TRUE),
-- Noir Corne; race: Oni; birth region: Higashi; https://arknights.wiki.gg/wiki/Noir_Corne
-- Noir Corne: Higashi; documented birthplace.
('Noir Corne', 'Noir Corne', 31, 'Male', 5, '1995-08-30', 25, 180, 2, 5, 0, 836.68, 580.65, TRUE),
-- Durin; race: Durin; birth region: Durin; https://arknights.wiki.gg/wiki/Durin
-- Durin: Durin; underground homeland background.
-- https://arknights.wiki.gg/wiki/Durin
('Durin', 'Durin', 25, 'Female', 4, '2000-12-15', 12, 131, 2, 4, 0, 204.66, 263.43, TRUE),
-- Additional couriers: ages, birth years and logistics roles are simulation values as of 2026-11-01.
-- Unconfirmed birth regions and Manticore's birthday are simulated; undisclosed names use codenames.
-- Angelina; race: Vulpo; birth region: Siracusa; https://arknights.wiki.gg/wiki/Angelina/Story
-- Angelina: Siracusa; upbringing and messenger background.
-- https://arknights.wiki.gg/wiki/Angelina/Story
('Angelina Ajimu', 'Angelina', 20, 'Female', 17, '2006-05-14', 36, 162, 2, 17, 0, 594.15, 394.14, TRUE),
-- Blue Poison; race: Anura; birth region: Iberia; https://arknights.wiki.gg/wiki/Blue_Poison/Story
-- Blue Poison: Iberia; confirmed upbringing despite the unknown birthplace in her basic profile.
-- https://arknights.wiki.gg/wiki/Blue_Poison/Operator_Record
('Blue Poison', 'Blue Poison', 24, 'Female', 6, '2002-01-18', 4, 157, 2, 6, 0, 611.52, 144.39, TRUE),
-- Platinum; race: Kuranta; birth region: Kazimierz; Pegasus subtype mapped to Kuranta; former Armorless Union member; https://arknights.wiki.gg/wiki/Platinum/Story
-- Platinum: Kazimierz; documented birthplace.
('Centaurea', 'Platinum', 22, 'Female', 8, '2004-02-13', 19, 160, 2, 8, 0, 408.92, 553.46, TRUE),
-- Goldenglow; race: Feline; birth region: Victoria; https://arknights.wiki.gg/wiki/Goldenglow/Story
-- Goldenglow: Victoria; documented birthplace.
('Susie Glitter', 'Goldenglow', 23, 'Female', 19, '2003-01-07', 15, 159, 2, 19, 0, 509.80, 426.40, TRUE),
-- Surtr; race: Sarkaz; simulated birth region: Kazdel; https://arknights.wiki.gg/wiki/Surtr
-- Surtr: Regional specialty left unspecified; no reliable single-region background selected.
('Surtr', 'Surtr', 25, 'Female', 7, '2001-01-22', 32, 162, 2, NULL, 0, 589.77, 493.70, TRUE),
-- Suzuran; race: Vulpo; birth region: Higashi; Nine-Tailed Vulpo subtype mapped to Vulpo; https://arknights.wiki.gg/wiki/Suzuran/Story
-- Suzuran: Higashi; documented birthplace.
('Lisa', 'Suzuran', 14, 'Female', 5, '2012-10-09', 36, 137, 2, 5, 0, 896.57, 574.17, TRUE),
-- Sussurro; race: Vulpo; birth region: Siracusa; https://arknights.wiki.gg/wiki/Sussurro/Story
-- Sussurro: Siracusa; documented birthplace.
('Sussurro', 'Sussurro', 25, 'Female', 17, '2001-09-04', 36, 142, 2, 17, 0, 602.00, 349.87, TRUE),
-- Myrrh; race: Vulpo; simulated birth region: Sargon; https://arknights.wiki.gg/wiki/Myrrh/Story
-- Myrrh: Regional specialty left unspecified; her profile does not establish a birthplace.
('Adelaide', 'Myrrh', 23, 'Female', 15, '2003-05-05', 36, 143, 2, NULL, 0, 576.80, 391.62, TRUE),
-- Manticore; race: Manticore; birth region: Sargon; simulated birthday; https://arknights.wiki.gg/wiki/Manticore/Story
-- Manticore: Sargon; documented birthplace and field activity.
-- https://arknights.wiki.gg/wiki/Manticore/File
('Manticore', 'Manticore', 24, 'Female', 15, '2002-06-01', 24, 155, 2, 15, 0, 267.69, 362.78, TRUE),
-- Ethan; race: Savra; simulated birth region: Columbia; https://arknights.wiki.gg/wiki/Ethan/Story
-- Ethan: Lungmen activity and LGD referral, represented by Yan; not the simulated Columbian birthplace.
-- https://arknights.wiki.gg/wiki/Ethan/Story
('Ethan', 'Ethan', 27, 'Male', 3, '1999-01-29', 33, 163, 2, 20, 0, 828.86, 510.79, TRUE),
-- Scavenger; race: Zalak; simulated birth region: Sargon; https://arknights.wiki.gg/wiki/Scavenger/Story
-- Scavenger: Regional specialty left unspecified; her profile does not establish a birthplace.
('Hamelin', 'Scavenger', 30, 'Female', 15, '1996-08-22', 37, 164, 2, NULL, 0, 604.73, 455.64, TRUE),
-- Dur-nar; race: Vulpo; birth region: Columbia; https://arknights.wiki.gg/wiki/Dur-nar/Story
-- Dur-nar: Columbia; documented birthplace.
('Makayla Springthunder', 'Dur-nar', 38, 'Female', 3, '1988-08-11', 36, 160, 2, 3, 0, 244.11, 478.14, TRUE),
-- Frostleaf; race: Vulpo; birth region: Columbia; https://arknights.wiki.gg/wiki/Frostleaf/Story
-- Frostleaf: Columbia; documented birthplace.
('Frostleaf', 'Frostleaf', 22, 'Female', 3, '2004-05-01', 36, 157, 2, 3, 0, 250.65, 567.59, TRUE),
-- Meteor; race: Kuranta; birth region: Kazimierz; https://arknights.wiki.gg/wiki/Meteor
-- Meteor: Kazimierz; documented birthplace.
('Meteor', 'Meteor', 29, 'Female', 8, '1997-05-01', 19, 164, 2, 8, 0, 465.97, 560.19, TRUE),
-- Vigna; race: Sarkaz; birth region: Kazdel; https://arknights.wiki.gg/wiki/Vigna/Story
-- Vigna: Kazdel; documented birthplace.
('Vigna', 'Vigna', 20, 'Female', 7, '2005-12-12', 32, 142, 2, 7, 0, 734.08, 490.56, TRUE),
-- Haze; race: Feline; birth region: Victoria; https://arknights.wiki.gg/wiki/Haze/Story
-- Haze: Victoria; documented birthplace.
('Haze', 'Haze', 28, 'Female', 19, '1998-01-02', 15, 158, 2, 19, 0, 478.06, 458.63, TRUE),
-- Matoimaru; race: Oni; birth region: Higashi; https://arknights.wiki.gg/wiki/Matoimaru
-- Matoimaru: Higashi; documented birthplace.
('Matoimaru', 'Matoimaru', 24, 'Female', 5, '2002-01-03', 25, 172, 2, 5, 0, 866.04, 580.86, TRUE),
-- Estelle; race: Archosauria; simulated birth region: Sargon; https://arknights.wiki.gg/wiki/Estelle/Story
-- Estelle: Regional specialty left unspecified; her profile does not disclose a birthplace.
('Estelle', 'Estelle', 22, 'Female', 15, '2004-02-14', 5, 162, 2, NULL, 0, 243.68, 464.78, TRUE),
-- Greyy; race: Perro; birth region: Bolivar; https://arknights.wiki.gg/wiki/Greyy
-- Greyy: Bolivar; documented birthplace.
('Greyy', 'Greyy', 19, 'Male', 2, '2007-06-06', 26, 163, 2, 2, 0, 132.41, 398.24, TRUE),
-- Folinic; race: Feline; birth region: Ursus; https://arknights.wiki.gg/wiki/Folinic/Story
-- Folinic: Ursus; documented birthplace.
('Louisa', 'Folinic', 22, 'Female', 18, '2004-08-18', 15, 164, 2, 18, 0, 622.20, 617.31, TRUE);

COMMIT;

USE `rhodev_island_logistic`;

BEGIN;

INSERT INTO `warehouse`
(`id`, `name`, `region_id`, `capacity`, `x_position`, `y_position`)
VALUES
-- Grouped by region (region id order). Warehouse ids are stable and referenced by `warehouse_inventory`.
  -- Aegir (region 1): 3 locations
  (13, 'Aegir Medical Transfer Depot', 1, 12000, 700.00, 95.00),
  (14, 'Aegir Deepwater Medical Reserve', 1, 14000, 765.00, 80.00),
  (15, 'Aegir Outer Sea Supply Depot', 1, 10000, 830.00, 90.00),
  -- Bolivar (region 2): 2 locations
  (16, 'Bolivar Central Medical Depot', 2, 14000, 120.00, 440.00),
  (17, 'Bolivar Western Relief Depot', 2, 10000, 100.00, 400.00),
  -- Columbia (region 3): 4 locations
  (1, 'Columbia Central Medical Depot', 3, 24000, 235.00, 480.00),
  (18, 'Columbia Northern Medical Depot', 3, 20000, 245.00, 545.00),
  (19, 'Columbia Eastern Distribution Hub', 3, 18000, 300.00, 490.00),
  (20, 'Columbia Frontier Medical Reserve', 3, 16000, 185.00, 465.00),
  -- Durin (region 4): 1 location
  (21, 'Durin Subterranean Medical Depot', 4, 8000, 220.00, 285.00),
  -- Higashi (region 5): 2 locations
  (22, 'Higashi Eastern Medical Depot', 5, 14000, 900.00, 590.00),
  (23, 'Higashi Western Medical Depot', 5, 14000, 855.00, 590.00),
  -- Iberia (region 6): 3 locations
  (10, 'Iberia Coastal Medical Depot', 6, 14000, 590.00, 145.00),
  (24, 'Iberia Inland Medical Reserve', 6, 12000, 540.00, 215.00),
  (25, 'Iberia Harbor Relief Depot', 6, 12000, 520.00, 145.00),
  -- Kazdel (region 7): 2 locations
  (11, 'Kazdel Emergency Supply Depot', 7, 16000, 695.00, 470.00),
  (26, 'Kazdel Reconstruction Medical Depot', 7, 14000, 745.00, 480.00),
  -- Kazimierz (region 8): 2 locations
  (6, 'Kazimierz Distribution Hub', 8, 18000, 405.00, 545.00),
  (27, 'Kazimierz Provincial Medical Reserve', 8, 16000, 435.00, 565.00),
  -- Kjerag (region 9): 1 location
  (12, 'Kjerag Mountain Medical Depot', 9, 10000, 370.00, 455.00),
  -- Laterano (region 10): 1 location
  (7, 'Laterano Medical Reserve', 10, 14000, 580.00, 275.00),
  -- Leithanien (region 11): 2 locations
  (8, 'Leithanien Medical Depot', 11, 16000, 560.00, 470.00),
  (28, 'Leithanien Southern Clinical Supply Depot', 11, 14000, 590.00, 465.00),
  -- Minos (region 12): 2 locations
  (29, 'Minos Central Medical Depot', 12, 12000, 320.00, 335.00),
  (30, 'Minos Rural Relief Depot', 12, 8000, 340.00, 305.00),
  -- Rim Billiton (region 13): 3 locations
  (5, 'Rim Billiton Field Supply Depot', 13, 16000, 710.00, 315.00),
  (31, 'Rim Billiton Mining District Medical Depot', 13, 14000, 735.00, 345.00),
  (32, 'Rim Billiton Remote Medical Reserve', 13, 10000, 710.00, 270.00),
  -- Sami (region 14): 3 locations
  (33, 'Sami Southern Medical Transfer Depot', 14, 10000, 380.00, 635.00),
  (34, 'Sami Frontier Medical Reserve', 14, 8000, 425.00, 675.00),
  (35, 'Sami Remote Relief Depot', 14, 6000, 355.00, 685.00),
  -- Sargon (region 15): 5 locations
  (9, 'Sargon Desert Relief Depot', 15, 14000, 200.00, 180.00),
  (36, 'Sargon Oasis Medical Depot', 15, 12000, 280.00, 220.00),
  (37, 'Sargon Rainforest Relief Depot', 15, 10000, 370.00, 160.00),
  (38, 'Sargon Northern Medical Hub', 15, 16000, 260.00, 365.00),
  (39, 'Sargon Remote Medical Reserve', 15, 10000, 140.00, 180.00),
  -- Siesta (region 16): 1 location
  (40, 'Siesta Mobile Medical Depot', 16, 8000, 400.00, 245.00),
  -- Siracusa (region 17): 2 locations
  (41, 'Siracusa Central Medical Depot', 17, 16000, 590.00, 375.00),
  (42, 'Siracusa Coastal Relief Depot', 17, 12000, 610.00, 350.00),
  -- Ursus (region 18): 6 locations
  (4, 'Ursus Northern Medical Depot', 18, 18000, 650.00, 680.00),
  (43, 'Ursus Central Medical Depot', 18, 24000, 610.00, 620.00),
  (44, 'Ursus Western Medical Reserve', 18, 18000, 510.00, 640.00),
  (45, 'Ursus Eastern Distribution Hub', 18, 20000, 750.00, 625.00),
  (46, 'Ursus Southern Medical Depot', 18, 22000, 670.00, 590.00),
  (47, 'Ursus Frontier Relief Depot', 18, 12000, 775.00, 655.00),
  -- Victoria (region 19): 4 locations
  (2, 'Victoria Regional Medical Depot', 19, 22000, 450.00, 410.00),
  (48, 'Victoria Central Medical Reserve', 19, 26000, 470.00, 440.00),
  (49, 'Victoria Eastern Medical Depot', 19, 18000, 505.00, 420.00),
  (50, 'Victoria Frontier Relief Depot', 19, 16000, 460.00, 370.00),
  -- Yan (region 20): 5 locations
  (3, 'Yan Eastern Medical Depot', 20, 24000, 890.00, 435.00),
  (51, 'Yan Central Medical Reserve', 20, 28000, 840.00, 435.00),
  (52, 'Yan Northern Medical Depot', 20, 22000, 850.00, 515.00),
  (53, 'Yan Southern Distribution Hub', 20, 20000, 865.00, 370.00),
  (54, 'Yan Western Medical Depot', 20, 22000, 810.00, 420.00),
  -- Tara (region 21): 1 location
  (55, 'Tara Regional Medical Reserve', 21, 12000, 423.00, 362.00);

COMMIT;

USE `rhodev_island_logistic`;

BEGIN;

INSERT INTO `order_status` (`id`, `status`) VALUES
(1, 'PENDING'),
(2, 'IN_PRODUCTION'),
(3, 'READY_FOR_PICKUP'),
(4, 'ASSIGNED'),
(5, 'IN_TRANSIT'),
(6, 'DELIVERED');

COMMIT;

USE `rhodev_island_logistic`;

BEGIN;

INSERT INTO `medicine_type` (`id`, `name`) VALUES
(1, 'Oripathy Suppressant'),
(2, 'Analgesic'),
(3, 'Anti-inflammatory'),
(4, 'Antimicrobial'),
(5, 'Antiseptic'),
(6, 'Fluid and Electrolyte Replacement'),
(7, 'Burn Treatment'),
(8, 'Respiratory Medicine'),
(9, 'Emergency Medicine'),
(10, 'Nutritional Supplement');

-- Manufacturer region IDs follow the `region` insert above.
INSERT INTO `manufacturer`
(`id`, `name`, `region_id`, `x_position`, `y_position`)
VALUES
(1, 'Columbia Relief Pharmaceuticals', 3, 245.00, 500.00),
(2, 'Victoria Emergency Medicines', 19, 475.00, 405.00),
(3, 'Yan Medical Works', 20, 860.00, 445.00),
(4, 'Ursus Clinical Supplies', 18, 620.00, 640.00),
(5, 'Rim Billiton Field Pharmaceuticals', 13, 690.00, 330.00),
(6, 'Kazimierz Medical Cooperative', 8, 400.00, 570.00),
(7, 'Laterano Health Laboratories', 10, 605.00, 270.00),
(8, 'Leithanien Therapeutics', 11, 575.00, 490.00),
(9, 'Sargon Relief Medicines', 15, 265.00, 190.00),
(10, 'Iberia Coastal Pharmaceuticals', 6, 550.00, 155.00),
(11, 'Kazdel Emergency Medical Works', 7, 715.00, 480.00),
(12, 'Kjerag Alpine Remedies', 9, 355.00, 435.00);

INSERT INTO `medicine`
(`id`, `name`, `medicine_type_id`, `dosage_form`, `manufacturer_id`, `description`)
VALUES
(1, 'Oripathy Suppressant Tablets', 1, 'Tablet', 1,
 'Oral suppressant formulation for routine Oripathy care requests.'),
(2, 'Oripathy Suppressant Injection', 1, 'Injection', 1,
 'Injectable suppressant formulation for field-clinic Oripathy care requests.'),
(3, 'Pain Relief Tablets', 2, 'Tablet', 2,
 'Tablet formulation for general pain-management supply requests.'),
(4, 'Pain Relief Oral Suspension', 2, 'Oral Suspension', 2,
 'Liquid oral formulation for the pain-management catalogue.'),
(5, 'Emergency Pain Relief Injection', 2, 'Injection', 2,
 'Injectable formulation stocked in the emergency pain-management catalogue.'),
(6, 'Anti-inflammatory Tablets', 3, 'Tablet', 3,
 'Oral formulation for inflammation-care supply requests.'),
(7, 'Anti-inflammatory Gel', 3, 'Gel', 3,
 'Topical formulation for the inflammation-care catalogue.'),
(8, 'Antimicrobial Capsules', 4, 'Capsule', 1,
 'Oral antimicrobial formulation for clinic and outbreak-response inventories.'),
(9, 'Antimicrobial Infusion', 4, 'Infusion', 1,
 'Infusion formulation stocked by field hospitals and regional clinics.'),
(10, 'Topical Antimicrobial Ointment', 4, 'Ointment', 3,
 'Topical antimicrobial formulation in the wound-care catalogue.'),
(11, 'Antiseptic Solution', 5, 'Solution', 2,
 'Antiseptic formulation stocked in clinic and field-care inventories.'),
(12, 'Antiseptic Spray', 5, 'Spray', 2,
 'Portable antiseptic formulation for field-care supply requests.'),
(13, 'Oral Rehydration Salts', 6, 'Powder', 3,
 'Powder sachets in the hydration-support catalogue.'),
(14, 'Electrolyte Infusion', 6, 'Infusion', 4,
 'Electrolyte formulation for field-hospital fluid-support inventories.'),
(15, 'Sterile Saline Infusion', 6, 'Infusion', 4,
 'Sterile saline formulation for the clinical fluid-supply catalogue.'),
(16, 'Burn Relief Gel', 7, 'Gel', 4,
 'Gel formulation stocked for fire and industrial-incident response.'),
(17, 'Burn Care Cream', 7, 'Cream', 4,
 'Cream formulation for the burn-care supply catalogue.'),
(18, 'Respiratory Relief Inhaler', 8, 'Inhalation Aerosol', 1,
 'Inhaler formulation in the respiratory-care catalogue.'),
(19, 'Respiratory Relief Nebulizer Solution', 8, 'Nebulizer Solution', 1,
 'Nebulizer formulation stocked by respiratory-care clinics.'),
(20, 'Emergency Hemostatic Injection', 9, 'Injection', 2,
 'Fictional hemostatic formulation for the emergency-response catalogue.'),
(21, 'Local Anesthetic Injection', 9, 'Injection', 2,
 'Injectable anesthetic formulation stocked by field hospitals.'),
(22, 'Nutritional Recovery Powder', 10, 'Powder', 3,
 'Powdered supplement for displacement and recovery-support inventories.'),
(23, 'Nutritional Recovery Oral Solution', 10, 'Oral Solution', 3,
 'Liquid supplement for the recovery-support catalogue.'),
(24, 'Multivitamin Tablets', 10, 'Tablet', 4,
 'Tablet supplement for routine clinic and relief-supply inventories.');

COMMIT;

USE `rhodev_island_logistic`;

BEGIN;

-- Initial stock per warehouse x medicine (every warehouse stocks the full 24-medicine catalogue).
-- Generated with seed 20261108: total stock per warehouse is roughly 35-55% of capacity (rounded to 5s, min 50).
-- Essentials (pain relief, ORS, antiseptic) are stocked deeper than injectables and emergency items.
-- reorder_threshold is about 35% of the initial quantity (rounded to 5s, min 10); the warehouse orders below it.
INSERT INTO `warehouse_inventory`
(`warehouse_id`, `medicine_id`, `quantity`, `reorder_threshold`)
VALUES
-- Warehouse 1: Columbia Central Medical Depot
(1, 1, 750, 260),
(1, 2, 365, 130),
(1, 3, 760, 265),
(1, 4, 480, 170),
(1, 5, 360, 125),
(1, 6, 755, 265),
(1, 7, 455, 160),
(1, 8, 805, 280),
(1, 9, 445, 155),
(1, 10, 445, 155),
(1, 11, 725, 255),
(1, 12, 565, 200),
(1, 13, 985, 345),
(1, 14, 545, 190),
(1, 15, 585, 205),
(1, 16, 450, 160),
(1, 17, 505, 175),
(1, 18, 615, 215),
(1, 19, 345, 120),
(1, 20, 300, 105),
(1, 21, 310, 110),
(1, 22, 620, 215),
(1, 23, 500, 175),
(1, 24, 695, 245),

-- Warehouse 2: Victoria Regional Medical Depot
(2, 1, 550, 190),
(2, 2, 255, 90),
(2, 3, 575, 200),
(2, 4, 305, 105),
(2, 5, 245, 85),
(2, 6, 515, 180),
(2, 7, 300, 105),
(2, 8, 495, 175),
(2, 9, 220, 75),
(2, 10, 350, 120),
(2, 11, 475, 165),
(2, 12, 345, 120),
(2, 13, 645, 225),
(2, 14, 320, 110),
(2, 15, 470, 165),
(2, 16, 290, 100),
(2, 17, 280, 100),
(2, 18, 400, 140),
(2, 19, 195, 70),
(2, 20, 155, 55),
(2, 21, 270, 95),
(2, 22, 400, 140),
(2, 23, 255, 90),
(2, 24, 425, 150),

-- Warehouse 3: Yan Eastern Medical Depot
(3, 1, 545, 190),
(3, 2, 290, 100),
(3, 3, 790, 275),
(3, 4, 420, 145),
(3, 5, 345, 120),
(3, 6, 720, 250),
(3, 7, 470, 165),
(3, 8, 615, 215),
(3, 9, 430, 150),
(3, 10, 470, 165),
(3, 11, 635, 220),
(3, 12, 455, 160),
(3, 13, 895, 315),
(3, 14, 425, 150),
(3, 15, 575, 200),
(3, 16, 490, 170),
(3, 17, 440, 155),
(3, 18, 525, 185),
(3, 19, 365, 130),
(3, 20, 255, 90),
(3, 21, 255, 90),
(3, 22, 495, 175),
(3, 23, 375, 130),
(3, 24, 480, 170),

-- Warehouse 4: Ursus Northern Medical Depot
(4, 1, 425, 150),
(4, 2, 220, 75),
(4, 3, 560, 195),
(4, 4, 305, 105),
(4, 5, 235, 80),
(4, 6, 470, 165),
(4, 7, 255, 90),
(4, 8, 415, 145),
(4, 9, 265, 95),
(4, 10, 350, 120),
(4, 11, 385, 135),
(4, 12, 260, 90),
(4, 13, 560, 195),
(4, 14, 255, 90),
(4, 15, 365, 130),
(4, 16, 250, 90),
(4, 17, 280, 100),
(4, 18, 350, 120),
(4, 19, 255, 90),
(4, 20, 160, 55),
(4, 21, 190, 65),
(4, 22, 420, 145),
(4, 23, 240, 85),
(4, 24, 390, 135),

-- Warehouse 5: Rim Billiton Field Supply Depot
(5, 1, 310, 110),
(5, 2, 210, 75),
(5, 3, 485, 170),
(5, 4, 210, 75),
(5, 5, 160, 55),
(5, 6, 405, 140),
(5, 7, 250, 90),
(5, 8, 350, 120),
(5, 9, 190, 65),
(5, 10, 210, 75),
(5, 11, 345, 120),
(5, 12, 310, 110),
(5, 13, 520, 180),
(5, 14, 210, 75),
(5, 15, 315, 110),
(5, 16, 195, 70),
(5, 17, 235, 80),
(5, 18, 245, 85),
(5, 19, 180, 65),
(5, 20, 140, 50),
(5, 21, 190, 65),
(5, 22, 365, 130),
(5, 23, 255, 90),
(5, 24, 310, 110),

-- Warehouse 6: Kazimierz Distribution Hub
(6, 1, 420, 145),
(6, 2, 190, 65),
(6, 3, 425, 150),
(6, 4, 275, 95),
(6, 5, 215, 75),
(6, 6, 360, 125),
(6, 7, 215, 75),
(6, 8, 430, 150),
(6, 9, 240, 85),
(6, 10, 320, 110),
(6, 11, 455, 160),
(6, 12, 280, 100),
(6, 13, 580, 205),
(6, 14, 260, 90),
(6, 15, 385, 135),
(6, 16, 250, 90),
(6, 17, 250, 90),
(6, 18, 265, 95),
(6, 19, 195, 70),
(6, 20, 165, 60),
(6, 21, 190, 65),
(6, 22, 400, 140),
(6, 23, 315, 110),
(6, 24, 315, 110),

-- Warehouse 7: Laterano Medical Reserve
(7, 1, 250, 90),
(7, 2, 170, 60),
(7, 3, 275, 95),
(7, 4, 160, 55),
(7, 5, 125, 45),
(7, 6, 340, 120),
(7, 7, 200, 70),
(7, 8, 280, 100),
(7, 9, 140, 50),
(7, 10, 175, 60),
(7, 11, 335, 115),
(7, 12, 215, 75),
(7, 13, 355, 125),
(7, 14, 180, 65),
(7, 15, 305, 105),
(7, 16, 225, 80),
(7, 17, 160, 55),
(7, 18, 225, 80),
(7, 19, 160, 55),
(7, 20, 110, 40),
(7, 21, 165, 60),
(7, 22, 215, 75),
(7, 23, 180, 65),
(7, 24, 260, 90),

-- Warehouse 8: Leithanien Medical Depot
(8, 1, 420, 145),
(8, 2, 215, 75),
(8, 3, 530, 185),
(8, 4, 220, 75),
(8, 5, 165, 60),
(8, 6, 315, 110),
(8, 7, 220, 75),
(8, 8, 420, 145),
(8, 9, 235, 80),
(8, 10, 280, 100),
(8, 11, 465, 165),
(8, 12, 275, 95),
(8, 13, 405, 140),
(8, 14, 240, 85),
(8, 15, 400, 140),
(8, 16, 260, 90),
(8, 17, 205, 70),
(8, 18, 275, 95),
(8, 19, 220, 75),
(8, 20, 165, 60),
(8, 21, 175, 60),
(8, 22, 410, 145),
(8, 23, 205, 70),
(8, 24, 325, 115),

-- Warehouse 9: Sargon Desert Relief Depot
(9, 1, 315, 110),
(9, 2, 210, 75),
(9, 3, 320, 110),
(9, 4, 185, 65),
(9, 5, 175, 60),
(9, 6, 330, 115),
(9, 7, 240, 85),
(9, 8, 315, 110),
(9, 9, 175, 60),
(9, 10, 260, 90),
(9, 11, 350, 120),
(9, 12, 270, 95),
(9, 13, 395, 140),
(9, 14, 215, 75),
(9, 15, 245, 85),
(9, 16, 260, 90),
(9, 17, 240, 85),
(9, 18, 250, 90),
(9, 19, 160, 55),
(9, 20, 120, 40),
(9, 21, 185, 65),
(9, 22, 270, 95),
(9, 23, 255, 90),
(9, 24, 270, 95),

-- Warehouse 10: Iberia Coastal Medical Depot
(10, 1, 350, 120),
(10, 2, 195, 70),
(10, 3, 315, 110),
(10, 4, 180, 65),
(10, 5, 160, 55),
(10, 6, 290, 100),
(10, 7, 205, 70),
(10, 8, 365, 130),
(10, 9, 175, 60),
(10, 10, 275, 95),
(10, 11, 300, 105),
(10, 12, 250, 90),
(10, 13, 385, 135),
(10, 14, 235, 80),
(10, 15, 255, 90),
(10, 16, 250, 90),
(10, 17, 175, 60),
(10, 18, 260, 90),
(10, 19, 190, 65),
(10, 20, 125, 45),
(10, 21, 185, 65),
(10, 22, 280, 100),
(10, 23, 220, 75),
(10, 24, 245, 85),

-- Warehouse 11: Kazdel Emergency Supply Depot
(11, 1, 420, 145),
(11, 2, 260, 90),
(11, 3, 510, 180),
(11, 4, 260, 90),
(11, 5, 195, 70),
(11, 6, 330, 115),
(11, 7, 230, 80),
(11, 8, 350, 120),
(11, 9, 195, 70),
(11, 10, 315, 110),
(11, 11, 460, 160),
(11, 12, 325, 115),
(11, 13, 520, 180),
(11, 14, 235, 80),
(11, 15, 300, 105),
(11, 16, 275, 95),
(11, 17, 310, 110),
(11, 18, 235, 80),
(11, 19, 215, 75),
(11, 20, 135, 45),
(11, 21, 175, 60),
(11, 22, 375, 130),
(11, 23, 280, 100),
(11, 24, 285, 100),

-- Warehouse 12: Kjerag Mountain Medical Depot
(12, 1, 235, 80),
(12, 2, 135, 45),
(12, 3, 260, 90),
(12, 4, 115, 40),
(12, 5, 100, 35),
(12, 6, 170, 60),
(12, 7, 125, 45),
(12, 8, 240, 85),
(12, 9, 100, 35),
(12, 10, 180, 65),
(12, 11, 240, 85),
(12, 12, 130, 45),
(12, 13, 265, 95),
(12, 14, 130, 45),
(12, 15, 220, 75),
(12, 16, 155, 55),
(12, 17, 130, 45),
(12, 18, 135, 45),
(12, 19, 105, 35),
(12, 20, 85, 30),
(12, 21, 85, 30),
(12, 22, 155, 55),
(12, 23, 150, 50),
(12, 24, 150, 50),

-- Warehouse 13: Aegir Medical Transfer Depot
(13, 1, 280, 100),
(13, 2, 110, 40),
(13, 3, 260, 90),
(13, 4, 160, 55),
(13, 5, 130, 45),
(13, 6, 265, 95),
(13, 7, 165, 60),
(13, 8, 195, 70),
(13, 9, 115, 40),
(13, 10, 190, 65),
(13, 11, 300, 105),
(13, 12, 160, 55),
(13, 13, 280, 100),
(13, 14, 185, 65),
(13, 15, 235, 80),
(13, 16, 145, 50),
(13, 17, 160, 55),
(13, 18, 160, 55),
(13, 19, 110, 40),
(13, 20, 85, 30),
(13, 21, 135, 45),
(13, 22, 190, 65),
(13, 23, 140, 50),
(13, 24, 200, 70),

-- Warehouse 14: Aegir Deepwater Medical Reserve
(14, 1, 375, 130),
(14, 2, 200, 70),
(14, 3, 395, 140),
(14, 4, 185, 65),
(14, 5, 150, 50),
(14, 6, 370, 130),
(14, 7, 240, 85),
(14, 8, 400, 140),
(14, 9, 210, 75),
(14, 10, 285, 100),
(14, 11, 395, 140),
(14, 12, 290, 100),
(14, 13, 365, 130),
(14, 14, 200, 70),
(14, 15, 290, 100),
(14, 16, 265, 95),
(14, 17, 265, 95),
(14, 18, 215, 75),
(14, 19, 150, 50),
(14, 20, 125, 45),
(14, 21, 170, 60),
(14, 22, 305, 105),
(14, 23, 195, 70),
(14, 24, 280, 100),

-- Warehouse 15: Aegir Outer Sea Supply Depot
(15, 1, 295, 105),
(15, 2, 145, 50),
(15, 3, 280, 100),
(15, 4, 200, 70),
(15, 5, 115, 40),
(15, 6, 280, 100),
(15, 7, 170, 60),
(15, 8, 255, 90),
(15, 9, 140, 50),
(15, 10, 180, 65),
(15, 11, 230, 80),
(15, 12, 190, 65),
(15, 13, 355, 125),
(15, 14, 175, 60),
(15, 15, 205, 70),
(15, 16, 140, 50),
(15, 17, 200, 70),
(15, 18, 205, 70),
(15, 19, 105, 35),
(15, 20, 105, 35),
(15, 21, 120, 40),
(15, 22, 190, 65),
(15, 23, 200, 70),
(15, 24, 180, 65),

-- Warehouse 16: Bolivar Central Medical Depot
(16, 1, 375, 130),
(16, 2, 160, 55),
(16, 3, 435, 150),
(16, 4, 170, 60),
(16, 5, 130, 45),
(16, 6, 295, 105),
(16, 7, 190, 65),
(16, 8, 360, 125),
(16, 9, 180, 65),
(16, 10, 200, 70),
(16, 11, 385, 135),
(16, 12, 220, 75),
(16, 13, 370, 130),
(16, 14, 175, 60),
(16, 15, 345, 120),
(16, 16, 200, 70),
(16, 17, 170, 60),
(16, 18, 240, 85),
(16, 19, 160, 55),
(16, 20, 150, 50),
(16, 21, 150, 50),
(16, 22, 330, 115),
(16, 23, 195, 70),
(16, 24, 280, 100),

-- Warehouse 17: Bolivar Western Relief Depot
(17, 1, 275, 95),
(17, 2, 105, 35),
(17, 3, 235, 80),
(17, 4, 140, 50),
(17, 5, 120, 40),
(17, 6, 200, 70),
(17, 7, 150, 50),
(17, 8, 200, 70),
(17, 9, 145, 50),
(17, 10, 180, 65),
(17, 11, 205, 70),
(17, 12, 145, 50),
(17, 13, 245, 85),
(17, 14, 165, 60),
(17, 15, 190, 65),
(17, 16, 150, 50),
(17, 17, 145, 50),
(17, 18, 165, 60),
(17, 19, 100, 35),
(17, 20, 80, 30),
(17, 21, 95, 35),
(17, 22, 215, 75),
(17, 23, 150, 50),
(17, 24, 155, 55),

-- Warehouse 18: Columbia Northern Medical Depot
(18, 1, 505, 175),
(18, 2, 300, 105),
(18, 3, 765, 270),
(18, 4, 380, 135),
(18, 5, 270, 95),
(18, 6, 560, 195),
(18, 7, 405, 140),
(18, 8, 640, 225),
(18, 9, 345, 120),
(18, 10, 350, 120),
(18, 11, 555, 195),
(18, 12, 420, 145),
(18, 13, 575, 200),
(18, 14, 420, 145),
(18, 15, 520, 180),
(18, 16, 365, 130),
(18, 17, 395, 140),
(18, 18, 410, 145),
(18, 19, 300, 105),
(18, 20, 245, 85),
(18, 21, 235, 80),
(18, 22, 580, 205),
(18, 23, 435, 150),
(18, 24, 430, 150),

-- Warehouse 19: Columbia Eastern Distribution Hub
(19, 1, 410, 145),
(19, 2, 235, 80),
(19, 3, 410, 145),
(19, 4, 250, 90),
(19, 5, 210, 75),
(19, 6, 330, 115),
(19, 7, 295, 105),
(19, 8, 400, 140),
(19, 9, 205, 70),
(19, 10, 235, 80),
(19, 11, 390, 135),
(19, 12, 315, 110),
(19, 13, 540, 190),
(19, 14, 240, 85),
(19, 15, 330, 115),
(19, 16, 255, 90),
(19, 17, 265, 95),
(19, 18, 225, 80),
(19, 19, 175, 60),
(19, 20, 165, 60),
(19, 21, 150, 50),
(19, 22, 370, 130),
(19, 23, 205, 70),
(19, 24, 315, 110),

-- Warehouse 20: Columbia Frontier Medical Reserve
(20, 1, 515, 180),
(20, 2, 255, 90),
(20, 3, 600, 210),
(20, 4, 350, 120),
(20, 5, 205, 70),
(20, 6, 400, 140),
(20, 7, 345, 120),
(20, 8, 435, 150),
(20, 9, 250, 90),
(20, 10, 345, 120),
(20, 11, 515, 180),
(20, 12, 300, 105),
(20, 13, 680, 240),
(20, 14, 370, 130),
(20, 15, 480, 170),
(20, 16, 330, 115),
(20, 17, 370, 130),
(20, 18, 305, 105),
(20, 19, 215, 75),
(20, 20, 225, 80),
(20, 21, 220, 75),
(20, 22, 460, 160),
(20, 23, 260, 90),
(20, 24, 340, 120),

-- Warehouse 21: Durin Subterranean Medical Depot
(21, 1, 195, 70),
(21, 2, 120, 40),
(21, 3, 215, 75),
(21, 4, 120, 40),
(21, 5, 115, 40),
(21, 6, 195, 70),
(21, 7, 130, 45),
(21, 8, 225, 80),
(21, 9, 145, 50),
(21, 10, 140, 50),
(21, 11, 270, 95),
(21, 12, 180, 65),
(21, 13, 220, 75),
(21, 14, 135, 45),
(21, 15, 230, 80),
(21, 16, 135, 45),
(21, 17, 145, 50),
(21, 18, 145, 50),
(21, 19, 105, 35),
(21, 20, 100, 35),
(21, 21, 95, 35),
(21, 22, 165, 60),
(21, 23, 140, 50),
(21, 24, 165, 60),

-- Warehouse 22: Higashi Eastern Medical Depot
(22, 1, 480, 170),
(22, 2, 235, 80),
(22, 3, 420, 145),
(22, 4, 230, 80),
(22, 5, 165, 60),
(22, 6, 375, 130),
(22, 7, 235, 80),
(22, 8, 380, 135),
(22, 9, 220, 75),
(22, 10, 250, 90),
(22, 11, 370, 130),
(22, 12, 320, 110),
(22, 13, 470, 165),
(22, 14, 250, 90),
(22, 15, 305, 105),
(22, 16, 255, 90),
(22, 17, 280, 100),
(22, 18, 245, 85),
(22, 19, 220, 75),
(22, 20, 140, 50),
(22, 21, 180, 65),
(22, 22, 390, 135),
(22, 23, 270, 95),
(22, 24, 355, 125),

-- Warehouse 23: Higashi Western Medical Depot
(23, 1, 340, 120),
(23, 2, 140, 50),
(23, 3, 410, 145),
(23, 4, 230, 80),
(23, 5, 120, 40),
(23, 6, 320, 110),
(23, 7, 225, 80),
(23, 8, 235, 80),
(23, 9, 170, 60),
(23, 10, 180, 65),
(23, 11, 265, 95),
(23, 12, 240, 85),
(23, 13, 435, 150),
(23, 14, 210, 75),
(23, 15, 275, 95),
(23, 16, 195, 70),
(23, 17, 215, 75),
(23, 18, 255, 90),
(23, 19, 150, 50),
(23, 20, 135, 45),
(23, 21, 125, 45),
(23, 22, 230, 80),
(23, 23, 200, 70),
(23, 24, 225, 80),

-- Warehouse 24: Iberia Inland Medical Reserve
(24, 1, 330, 115),
(24, 2, 145, 50),
(24, 3, 340, 120),
(24, 4, 145, 50),
(24, 5, 155, 55),
(24, 6, 250, 90),
(24, 7, 210, 75),
(24, 8, 295, 105),
(24, 9, 180, 65),
(24, 10, 230, 80),
(24, 11, 315, 110),
(24, 12, 235, 80),
(24, 13, 315, 110),
(24, 14, 165, 60),
(24, 15, 285, 100),
(24, 16, 205, 70),
(24, 17, 190, 65),
(24, 18, 205, 70),
(24, 19, 110, 40),
(24, 20, 115, 40),
(24, 21, 145, 50),
(24, 22, 240, 85),
(24, 23, 210, 75),
(24, 24, 205, 70),

-- Warehouse 25: Iberia Harbor Relief Depot
(25, 1, 280, 100),
(25, 2, 155, 55),
(25, 3, 275, 95),
(25, 4, 170, 60),
(25, 5, 110, 40),
(25, 6, 225, 80),
(25, 7, 185, 65),
(25, 8, 295, 105),
(25, 9, 160, 55),
(25, 10, 195, 70),
(25, 11, 330, 115),
(25, 12, 200, 70),
(25, 13, 390, 135),
(25, 14, 195, 70),
(25, 15, 200, 70),
(25, 16, 170, 60),
(25, 17, 165, 60),
(25, 18, 235, 80),
(25, 19, 140, 50),
(25, 20, 120, 40),
(25, 21, 110, 40),
(25, 22, 220, 75),
(25, 23, 190, 65),
(25, 24, 265, 95),

-- Warehouse 26: Kazdel Reconstruction Medical Depot
(26, 1, 395, 140),
(26, 2, 185, 65),
(26, 3, 385, 135),
(26, 4, 170, 60),
(26, 5, 140, 50),
(26, 6, 355, 125),
(26, 7, 205, 70),
(26, 8, 265, 95),
(26, 9, 210, 75),
(26, 10, 230, 80),
(26, 11, 390, 135),
(26, 12, 195, 70),
(26, 13, 455, 160),
(26, 14, 185, 65),
(26, 15, 260, 90),
(26, 16, 215, 75),
(26, 17, 190, 65),
(26, 18, 275, 95),
(26, 19, 145, 50),
(26, 20, 130, 45),
(26, 21, 185, 65),
(26, 22, 265, 95),
(26, 23, 220, 75),
(26, 24, 270, 95),

-- Warehouse 27: Kazimierz Provincial Medical Reserve
(27, 1, 385, 135),
(27, 2, 235, 80),
(27, 3, 350, 120),
(27, 4, 285, 100),
(27, 5, 190, 65),
(27, 6, 425, 150),
(27, 7, 285, 100),
(27, 8, 415, 145),
(27, 9, 235, 80),
(27, 10, 235, 80),
(27, 11, 440, 155),
(27, 12, 260, 90),
(27, 13, 445, 155),
(27, 14, 255, 90),
(27, 15, 275, 95),
(27, 16, 280, 100),
(27, 17, 195, 70),
(27, 18, 230, 80),
(27, 19, 185, 65),
(27, 20, 135, 45),
(27, 21, 215, 75),
(27, 22, 340, 120),
(27, 23, 215, 75),
(27, 24, 290, 100),

-- Warehouse 28: Leithanien Southern Clinical Supply Depot
(28, 1, 405, 140),
(28, 2, 190, 65),
(28, 3, 470, 165),
(28, 4, 255, 90),
(28, 5, 170, 60),
(28, 6, 325, 115),
(28, 7, 265, 95),
(28, 8, 405, 140),
(28, 9, 235, 80),
(28, 10, 265, 95),
(28, 11, 390, 135),
(28, 12, 300, 105),
(28, 13, 425, 150),
(28, 14, 215, 75),
(28, 15, 275, 95),
(28, 16, 280, 100),
(28, 17, 260, 90),
(28, 18, 300, 105),
(28, 19, 180, 65),
(28, 20, 180, 65),
(28, 21, 195, 70),
(28, 22, 405, 140),
(28, 23, 250, 90),
(28, 24, 295, 105),

-- Warehouse 29: Minos Central Medical Depot
(29, 1, 345, 120),
(29, 2, 205, 70),
(29, 3, 335, 115),
(29, 4, 210, 75),
(29, 5, 130, 45),
(29, 6, 270, 95),
(29, 7, 225, 80),
(29, 8, 375, 130),
(29, 9, 165, 60),
(29, 10, 210, 75),
(29, 11, 295, 105),
(29, 12, 200, 70),
(29, 13, 335, 115),
(29, 14, 220, 75),
(29, 15, 275, 95),
(29, 16, 195, 70),
(29, 17, 205, 70),
(29, 18, 210, 75),
(29, 19, 175, 60),
(29, 20, 140, 50),
(29, 21, 170, 60),
(29, 22, 340, 120),
(29, 23, 190, 65),
(29, 24, 290, 100),

-- Warehouse 30: Minos Rural Relief Depot
(30, 1, 200, 70),
(30, 2, 130, 45),
(30, 3, 215, 75),
(30, 4, 150, 50),
(30, 5, 80, 30),
(30, 6, 225, 80),
(30, 7, 105, 35),
(30, 8, 225, 80),
(30, 9, 90, 30),
(30, 10, 155, 55),
(30, 11, 190, 65),
(30, 12, 150, 50),
(30, 13, 220, 75),
(30, 14, 145, 50),
(30, 15, 195, 70),
(30, 16, 135, 45),
(30, 17, 150, 50),
(30, 18, 120, 40),
(30, 19, 110, 40),
(30, 20, 70, 25),
(30, 21, 115, 40),
(30, 22, 160, 55),
(30, 23, 125, 45),
(30, 24, 140, 50),

-- Warehouse 31: Rim Billiton Mining District Medical Depot
(31, 1, 375, 130),
(31, 2, 185, 65),
(31, 3, 385, 135),
(31, 4, 210, 75),
(31, 5, 150, 50),
(31, 6, 265, 95),
(31, 7, 195, 70),
(31, 8, 285, 100),
(31, 9, 150, 50),
(31, 10, 205, 70),
(31, 11, 330, 115),
(31, 12, 225, 80),
(31, 13, 365, 130),
(31, 14, 215, 75),
(31, 15, 295, 105),
(31, 16, 195, 70),
(31, 17, 200, 70),
(31, 18, 225, 80),
(31, 19, 140, 50),
(31, 20, 145, 50),
(31, 21, 160, 55),
(31, 22, 275, 95),
(31, 23, 165, 60),
(31, 24, 245, 85),

-- Warehouse 32: Rim Billiton Remote Medical Reserve
(32, 1, 210, 75),
(32, 2, 125, 45),
(32, 3, 265, 95),
(32, 4, 145, 50),
(32, 5, 115, 40),
(32, 6, 215, 75),
(32, 7, 165, 60),
(32, 8, 265, 95),
(32, 9, 125, 45),
(32, 10, 200, 70),
(32, 11, 280, 100),
(32, 12, 190, 65),
(32, 13, 310, 110),
(32, 14, 125, 45),
(32, 15, 245, 85),
(32, 16, 150, 50),
(32, 17, 175, 60),
(32, 18, 145, 50),
(32, 19, 95, 35),
(32, 20, 100, 35),
(32, 21, 110, 40),
(32, 22, 195, 70),
(32, 23, 120, 40),
(32, 24, 215, 75),

-- Warehouse 33: Sami Southern Medical Transfer Depot
(33, 1, 290, 100),
(33, 2, 190, 65),
(33, 3, 275, 95),
(33, 4, 170, 60),
(33, 5, 130, 45),
(33, 6, 305, 105),
(33, 7, 170, 60),
(33, 8, 250, 90),
(33, 9, 150, 50),
(33, 10, 205, 70),
(33, 11, 265, 95),
(33, 12, 245, 85),
(33, 13, 320, 110),
(33, 14, 180, 65),
(33, 15, 210, 75),
(33, 16, 205, 70),
(33, 17, 155, 55),
(33, 18, 180, 65),
(33, 19, 145, 50),
(33, 20, 140, 50),
(33, 21, 155, 55),
(33, 22, 310, 110),
(33, 23, 200, 70),
(33, 24, 195, 70),

-- Warehouse 34: Sami Frontier Medical Reserve
(34, 1, 175, 60),
(34, 2, 105, 35),
(34, 3, 205, 70),
(34, 4, 110, 40),
(34, 5, 80, 30),
(34, 6, 160, 55),
(34, 7, 105, 35),
(34, 8, 165, 60),
(34, 9, 120, 40),
(34, 10, 120, 40),
(34, 11, 170, 60),
(34, 12, 120, 40),
(34, 13, 250, 90),
(34, 14, 100, 35),
(34, 15, 175, 60),
(34, 16, 120, 40),
(34, 17, 110, 40),
(34, 18, 140, 50),
(34, 19, 90, 30),
(34, 20, 70, 25),
(34, 21, 75, 25),
(34, 22, 135, 45),
(34, 23, 115, 40),
(34, 24, 150, 50),

-- Warehouse 35: Sami Remote Relief Depot
(35, 1, 120, 40),
(35, 2, 80, 30),
(35, 3, 145, 50),
(35, 4, 85, 30),
(35, 5, 60, 20),
(35, 6, 130, 45),
(35, 7, 70, 25),
(35, 8, 100, 35),
(35, 9, 75, 25),
(35, 10, 100, 35),
(35, 11, 120, 40),
(35, 12, 80, 30),
(35, 13, 170, 60),
(35, 14, 80, 30),
(35, 15, 125, 45),
(35, 16, 75, 25),
(35, 17, 95, 35),
(35, 18, 105, 35),
(35, 19, 50, 20),
(35, 20, 50, 20),
(35, 21, 60, 20),
(35, 22, 100, 35),
(35, 23, 65, 25),
(35, 24, 115, 40),

-- Warehouse 36: Sargon Oasis Medical Depot
(36, 1, 315, 110),
(36, 2, 170, 60),
(36, 3, 330, 115),
(36, 4, 230, 80),
(36, 5, 145, 50),
(36, 6, 260, 90),
(36, 7, 235, 80),
(36, 8, 310, 110),
(36, 9, 200, 70),
(36, 10, 245, 85),
(36, 11, 285, 100),
(36, 12, 230, 80),
(36, 13, 420, 145),
(36, 14, 230, 80),
(36, 15, 325, 115),
(36, 16, 200, 70),
(36, 17, 180, 65),
(36, 18, 195, 70),
(36, 19, 140, 50),
(36, 20, 135, 45),
(36, 21, 150, 50),
(36, 22, 315, 110),
(36, 23, 175, 60),
(36, 24, 225, 80),

-- Warehouse 37: Sargon Rainforest Relief Depot
(37, 1, 255, 90),
(37, 2, 165, 60),
(37, 3, 325, 115),
(37, 4, 160, 55),
(37, 5, 125, 45),
(37, 6, 240, 85),
(37, 7, 200, 70),
(37, 8, 310, 110),
(37, 9, 175, 60),
(37, 10, 235, 80),
(37, 11, 290, 100),
(37, 12, 215, 75),
(37, 13, 290, 100),
(37, 14, 215, 75),
(37, 15, 240, 85),
(37, 16, 190, 65),
(37, 17, 165, 60),
(37, 18, 225, 80),
(37, 19, 165, 60),
(37, 20, 95, 35),
(37, 21, 115, 40),
(37, 22, 300, 105),
(37, 23, 150, 50),
(37, 24, 215, 75),

-- Warehouse 38: Sargon Northern Medical Hub
(38, 1, 405, 140),
(38, 2, 240, 85),
(38, 3, 430, 150),
(38, 4, 255, 90),
(38, 5, 165, 60),
(38, 6, 375, 130),
(38, 7, 260, 90),
(38, 8, 350, 120),
(38, 9, 225, 80),
(38, 10, 310, 110),
(38, 11, 305, 105),
(38, 12, 225, 80),
(38, 13, 425, 150),
(38, 14, 195, 70),
(38, 15, 340, 120),
(38, 16, 225, 80),
(38, 17, 215, 75),
(38, 18, 255, 90),
(38, 19, 175, 60),
(38, 20, 145, 50),
(38, 21, 200, 70),
(38, 22, 320, 110),
(38, 23, 215, 75),
(38, 24, 300, 105),

-- Warehouse 39: Sargon Remote Medical Reserve
(39, 1, 280, 100),
(39, 2, 140, 50),
(39, 3, 315, 110),
(39, 4, 125, 45),
(39, 5, 130, 45),
(39, 6, 245, 85),
(39, 7, 145, 50),
(39, 8, 230, 80),
(39, 9, 115, 40),
(39, 10, 160, 55),
(39, 11, 255, 90),
(39, 12, 160, 55),
(39, 13, 245, 85),
(39, 14, 145, 50),
(39, 15, 165, 60),
(39, 16, 150, 50),
(39, 17, 145, 50),
(39, 18, 150, 50),
(39, 19, 110, 40),
(39, 20, 90, 30),
(39, 21, 105, 35),
(39, 22, 235, 80),
(39, 23, 125, 45),
(39, 24, 210, 75),

-- Warehouse 40: Siesta Mobile Medical Depot
(40, 1, 200, 70),
(40, 2, 105, 35),
(40, 3, 200, 70),
(40, 4, 125, 45),
(40, 5, 80, 30),
(40, 6, 155, 55),
(40, 7, 130, 45),
(40, 8, 175, 60),
(40, 9, 80, 30),
(40, 10, 145, 50),
(40, 11, 155, 55),
(40, 12, 130, 45),
(40, 13, 170, 60),
(40, 14, 90, 30),
(40, 15, 160, 55),
(40, 16, 90, 30),
(40, 17, 95, 35),
(40, 18, 110, 40),
(40, 19, 65, 25),
(40, 20, 60, 20),
(40, 21, 65, 25),
(40, 22, 150, 50),
(40, 23, 125, 45),
(40, 24, 155, 55),

-- Warehouse 41: Siracusa Central Medical Depot
(41, 1, 340, 120),
(41, 2, 185, 65),
(41, 3, 305, 105),
(41, 4, 220, 75),
(41, 5, 135, 45),
(41, 6, 280, 100),
(41, 7, 215, 75),
(41, 8, 275, 95),
(41, 9, 165, 60),
(41, 10, 250, 90),
(41, 11, 345, 120),
(41, 12, 255, 90),
(41, 13, 430, 150),
(41, 14, 210, 75),
(41, 15, 330, 115),
(41, 16, 215, 75),
(41, 17, 235, 80),
(41, 18, 240, 85),
(41, 19, 160, 55),
(41, 20, 155, 55),
(41, 21, 135, 45),
(41, 22, 300, 105),
(41, 23, 180, 65),
(41, 24, 265, 95),

-- Warehouse 42: Siracusa Coastal Relief Depot
(42, 1, 275, 95),
(42, 2, 120, 40),
(42, 3, 315, 110),
(42, 4, 195, 70),
(42, 5, 110, 40),
(42, 6, 275, 95),
(42, 7, 145, 50),
(42, 8, 255, 90),
(42, 9, 165, 60),
(42, 10, 180, 65),
(42, 11, 235, 80),
(42, 12, 200, 70),
(42, 13, 360, 125),
(42, 14, 140, 50),
(42, 15, 250, 90),
(42, 16, 165, 60),
(42, 17, 195, 70),
(42, 18, 170, 60),
(42, 19, 115, 40),
(42, 20, 105, 35),
(42, 21, 120, 40),
(42, 22, 275, 95),
(42, 23, 140, 50),
(42, 24, 195, 70),

-- Warehouse 43: Ursus Central Medical Depot
(43, 1, 690, 240),
(43, 2, 435, 150),
(43, 3, 640, 225),
(43, 4, 490, 170),
(43, 5, 345, 120),
(43, 6, 635, 220),
(43, 7, 385, 135),
(43, 8, 685, 240),
(43, 9, 340, 120),
(43, 10, 525, 185),
(43, 11, 830, 290),
(43, 12, 585, 205),
(43, 13, 895, 315),
(43, 14, 430, 150),
(43, 15, 610, 215),
(43, 16, 480, 170),
(43, 17, 425, 150),
(43, 18, 585, 205),
(43, 19, 390, 135),
(43, 20, 315, 110),
(43, 21, 355, 125),
(43, 22, 620, 215),
(43, 23, 510, 180),
(43, 24, 555, 195),

-- Warehouse 44: Ursus Western Medical Reserve
(44, 1, 430, 150),
(44, 2, 250, 90),
(44, 3, 430, 150),
(44, 4, 245, 85),
(44, 5, 215, 75),
(44, 6, 435, 150),
(44, 7, 310, 110),
(44, 8, 440, 155),
(44, 9, 255, 90),
(44, 10, 325, 115),
(44, 11, 380, 135),
(44, 12, 340, 120),
(44, 13, 595, 210),
(44, 14, 245, 85),
(44, 15, 340, 120),
(44, 16, 330, 115),
(44, 17, 305, 105),
(44, 18, 325, 115),
(44, 19, 235, 80),
(44, 20, 150, 50),
(44, 21, 195, 70),
(44, 22, 345, 120),
(44, 23, 330, 115),
(44, 24, 320, 110),

-- Warehouse 45: Ursus Eastern Distribution Hub
(45, 1, 480, 170),
(45, 2, 220, 75),
(45, 3, 370, 130),
(45, 4, 245, 85),
(45, 5, 190, 65),
(45, 6, 335, 115),
(45, 7, 250, 90),
(45, 8, 330, 115),
(45, 9, 230, 80),
(45, 10, 320, 110),
(45, 11, 335, 115),
(45, 12, 245, 85),
(45, 13, 390, 135),
(45, 14, 275, 95),
(45, 15, 290, 100),
(45, 16, 290, 100),
(45, 17, 210, 75),
(45, 18, 285, 100),
(45, 19, 185, 65),
(45, 20, 150, 50),
(45, 21, 185, 65),
(45, 22, 305, 105),
(45, 23, 300, 105),
(45, 24, 305, 105),

-- Warehouse 46: Ursus Southern Medical Depot
(46, 1, 445, 155),
(46, 2, 320, 110),
(46, 3, 600, 210),
(46, 4, 360, 125),
(46, 5, 225, 80),
(46, 6, 420, 145),
(46, 7, 390, 135),
(46, 8, 480, 170),
(46, 9, 305, 105),
(46, 10, 435, 150),
(46, 11, 630, 220),
(46, 12, 305, 105),
(46, 13, 605, 210),
(46, 14, 275, 95),
(46, 15, 470, 165),
(46, 16, 340, 120),
(46, 17, 275, 95),
(46, 18, 320, 110),
(46, 19, 290, 100),
(46, 20, 230, 80),
(46, 21, 220, 75),
(46, 22, 385, 135),
(46, 23, 375, 130),
(46, 24, 405, 140),

-- Warehouse 47: Ursus Frontier Relief Depot
(47, 1, 315, 110),
(47, 2, 170, 60),
(47, 3, 380, 135),
(47, 4, 185, 65),
(47, 5, 165, 60),
(47, 6, 240, 85),
(47, 7, 220, 75),
(47, 8, 245, 85),
(47, 9, 130, 45),
(47, 10, 175, 60),
(47, 11, 250, 90),
(47, 12, 210, 75),
(47, 13, 385, 135),
(47, 14, 150, 50),
(47, 15, 260, 90),
(47, 16, 200, 70),
(47, 17, 155, 55),
(47, 18, 235, 80),
(47, 19, 120, 40),
(47, 20, 100, 35),
(47, 21, 135, 45),
(47, 22, 250, 90),
(47, 23, 220, 75),
(47, 24, 260, 90),

-- Warehouse 48: Victoria Central Medical Reserve
(48, 1, 750, 260),
(48, 2, 335, 115),
(48, 3, 690, 240),
(48, 4, 500, 175),
(48, 5, 320, 110),
(48, 6, 580, 205),
(48, 7, 455, 160),
(48, 8, 660, 230),
(48, 9, 440, 155),
(48, 10, 490, 170),
(48, 11, 780, 275),
(48, 12, 400, 140),
(48, 13, 775, 270),
(48, 14, 410, 145),
(48, 15, 640, 225),
(48, 16, 425, 150),
(48, 17, 350, 120),
(48, 18, 425, 150),
(48, 19, 305, 105),
(48, 20, 250, 90),
(48, 21, 370, 130),
(48, 22, 535, 185),
(48, 23, 380, 135),
(48, 24, 545, 190),

-- Warehouse 49: Victoria Eastern Medical Depot
(49, 1, 360, 125),
(49, 2, 225, 80),
(49, 3, 425, 150),
(49, 4, 210, 75),
(49, 5, 180, 65),
(49, 6, 375, 130),
(49, 7, 215, 75),
(49, 8, 390, 135),
(49, 9, 200, 70),
(49, 10, 305, 105),
(49, 11, 325, 115),
(49, 12, 325, 115),
(49, 13, 520, 180),
(49, 14, 260, 90),
(49, 15, 395, 140),
(49, 16, 270, 95),
(49, 17, 260, 90),
(49, 18, 320, 110),
(49, 19, 195, 70),
(49, 20, 165, 60),
(49, 21, 210, 75),
(49, 22, 300, 105),
(49, 23, 240, 85),
(49, 24, 360, 125),

-- Warehouse 50: Victoria Frontier Relief Depot
(50, 1, 485, 170),
(50, 2, 205, 70),
(50, 3, 505, 175),
(50, 4, 275, 95),
(50, 5, 175, 60),
(50, 6, 375, 130),
(50, 7, 255, 90),
(50, 8, 415, 145),
(50, 9, 220, 75),
(50, 10, 300, 105),
(50, 11, 375, 130),
(50, 12, 240, 85),
(50, 13, 435, 150),
(50, 14, 230, 80),
(50, 15, 415, 145),
(50, 16, 275, 95),
(50, 17, 295, 105),
(50, 18, 315, 110),
(50, 19, 210, 75),
(50, 20, 190, 65),
(50, 21, 175, 60),
(50, 22, 310, 110),
(50, 23, 290, 100),
(50, 24, 280, 100),

-- Warehouse 51: Yan Central Medical Reserve
(51, 1, 685, 240),
(51, 2, 440, 155),
(51, 3, 850, 300),
(51, 4, 475, 165),
(51, 5, 280, 100),
(51, 6, 715, 250),
(51, 7, 490, 170),
(51, 8, 760, 265),
(51, 9, 450, 160),
(51, 10, 460, 160),
(51, 11, 760, 265),
(51, 12, 615, 215),
(51, 13, 895, 315),
(51, 14, 535, 185),
(51, 15, 550, 190),
(51, 16, 475, 165),
(51, 17, 385, 135),
(51, 18, 615, 215),
(51, 19, 405, 140),
(51, 20, 270, 95),
(51, 21, 320, 110),
(51, 22, 685, 240),
(51, 23, 410, 145),
(51, 24, 670, 235),

-- Warehouse 52: Yan Northern Medical Depot
(52, 1, 680, 240),
(52, 2, 370, 130),
(52, 3, 705, 245),
(52, 4, 385, 135),
(52, 5, 295, 105),
(52, 6, 530, 185),
(52, 7, 315, 110),
(52, 8, 515, 180),
(52, 9, 315, 110),
(52, 10, 390, 135),
(52, 11, 730, 255),
(52, 12, 395, 140),
(52, 13, 725, 255),
(52, 14, 385, 135),
(52, 15, 575, 200),
(52, 16, 305, 105),
(52, 17, 450, 160),
(52, 18, 355, 125),
(52, 19, 305, 105),
(52, 20, 260, 90),
(52, 21, 320, 110),
(52, 22, 545, 190),
(52, 23, 355, 125),
(52, 24, 550, 190),

-- Warehouse 53: Yan Southern Distribution Hub
(53, 1, 580, 205),
(53, 2, 250, 90),
(53, 3, 470, 165),
(53, 4, 310, 110),
(53, 5, 215, 75),
(53, 6, 460, 160),
(53, 7, 300, 105),
(53, 8, 475, 165),
(53, 9, 255, 90),
(53, 10, 310, 110),
(53, 11, 400, 140),
(53, 12, 385, 135),
(53, 13, 465, 165),
(53, 14, 340, 120),
(53, 15, 390, 135),
(53, 16, 320, 110),
(53, 17, 310, 110),
(53, 18, 330, 115),
(53, 19, 205, 70),
(53, 20, 185, 65),
(53, 21, 250, 90),
(53, 22, 375, 130),
(53, 23, 330, 115),
(53, 24, 435, 150),

-- Warehouse 54: Yan Western Medical Depot
(54, 1, 605, 210),
(54, 2, 335, 115),
(54, 3, 635, 220),
(54, 4, 470, 165),
(54, 5, 285, 100),
(54, 6, 675, 235),
(54, 7, 365, 130),
(54, 8, 725, 255),
(54, 9, 380, 135),
(54, 10, 560, 195),
(54, 11, 635, 220),
(54, 12, 535, 185),
(54, 13, 735, 255),
(54, 14, 455, 160),
(54, 15, 500, 175),
(54, 16, 500, 175),
(54, 17, 345, 120),
(54, 18, 440, 155),
(54, 19, 260, 90),
(54, 20, 220, 75),
(54, 21, 355, 125),
(54, 22, 525, 185),
(54, 23, 450, 160),
(54, 24, 540, 190),

-- Warehouse 55: Tara Regional Medical Reserve
(55, 1, 275, 95),
(55, 2, 120, 40),
(55, 3, 260, 90),
(55, 4, 175, 60),
(55, 5, 140, 50),
(55, 6, 225, 80),
(55, 7, 135, 45),
(55, 8, 230, 80),
(55, 9, 165, 60),
(55, 10, 205, 70),
(55, 11, 305, 105),
(55, 12, 160, 55),
(55, 13, 280, 100),
(55, 14, 170, 60),
(55, 15, 185, 65),
(55, 16, 200, 70),
(55, 17, 150, 50),
(55, 18, 170, 60),
(55, 19, 145, 50),
(55, 20, 90, 30),
(55, 21, 145, 50),
(55, 22, 185, 65),
(55, 23, 165, 60),
(55, 24, 235, 80);

COMMIT;

USE `rhodev_island_logistic`;

BEGIN;

INSERT INTO `event_type` (`id`, `name`, `risk_level`) VALUES
(1, 'Catastrophe', 5),
(2, 'Originium Exposure Incident', 4),
(3, 'Oripathy Case Surge', 4),
(4, 'Disease Outbreak', 4),
(5, 'Armed Conflict', 5),
(6, 'Civil Unrest', 2),
(7, 'Population Displacement', 3),
(8, 'Mining Accident', 2),
(9, 'Industrial Accident', 2),
(10, 'Transport Accident', 1),
(11, 'Fire and Explosion', 3),
(12, 'Food and Water Contamination', 3);

COMMIT;
