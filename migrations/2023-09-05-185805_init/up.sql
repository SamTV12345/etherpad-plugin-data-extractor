-- Your SQL goes here
CREATE TABLE plugins(
    name TEXT NOT NULL PRIMARY KEY,
    description TEXT NOT NULL,
    time timestamp NOT NULL,
    version TEXT NOT NULL,
    official BOOLEAN NOT NULL
);


CREATE TABLE datas(
   id TEXT NOT NULL PRIMARY KEY,
   plugin_name TEXT NOT NULL,
  _id TEXT NOT NULL,
  _rev TEXT NOT NULL,
  name TEXT NOT NULL,
  license TEXT,
  downloads INTEGER NOT NULL,
  FOREIGN KEY (plugin_name) REFERENCES plugins(name)
);

CREATE TABLE versions(
    id TEXT NOT NULL PRIMARY KEY,
    data_id TEXT NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT NOT NULL,
    time timestamp NOT NULL,
    author_name TEXT NOT NULL,
    author_email TEXT NOT NULL,
    license TEXT,
    repository_type TEXT,
    repository_url TEXT,
    keywords TEXT,
    image TEXT,
    readme TEXT,
    FOREIGN KEY (data_id) REFERENCES datas(id)
);

CREATE TABLE keywords(
    id TEXT NOT NULL PRIMARY KEY,
    version_id TEXT NOT NULL,
    keyword TEXT NOT NULL,
    FOREIGN KEY (version_id) REFERENCES versions(id)
);


CREATE TABLE officialRepositories(
    id TEXT NOT  NULL PRIMARY KEY
);

CREATE INDEX versions_name ON versions(name);
CREATE INDEX datas_name ON datas(name);


CREATE TABLE timestamp_sync(
       id TEXT NOT NULL PRIMARY KEY,
       timestamp Timestamp NOT NULL
);