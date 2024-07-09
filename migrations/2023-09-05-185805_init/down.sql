-- This file should undo anything in `up.sql`
DROP TABLE plugins;
DROP TABLE datas;
DROP TABLE versions;

DROP INDEX versions_name;
DROP INDEX datas_name;
DROP TABLE officialRepositories;
DROP TABLE timestamp_sync;