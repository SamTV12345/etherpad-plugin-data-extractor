-- Your SQL goes here
CREATE TABLE plugins(
    name TEXT NOT NULL PRIMARY KEY,
    description TEXT NOT NULL,
    time DATETIME NOT NULL,
    version TEXT NOT NULL,
    official BOOLEAN NOT NULL
);


CREATE TABLE datas(
   id TEXT NOT NULL PRIMARY KEY,
   plugin_name TEXT NOT NULL,
  _id TEXT NOT NULL,
  _rev TEXT NOT NULL,
  name TEXT NOT NULL,
  license TEXT NOT NULL,
  downloads INTEGER NOT NULL,
  FOREIGN KEY (plugin_name) REFERENCES plugin(name)
);

CREATE TABLE versions(
    id TEXT NOT NULL PRIMARY KEY,
    data_id TEXT NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT NOT NULL,
    time DATETIME NOT NULL,
    author_name TEXT NOT NULL,
    author_email TEXT NOT NULL,
    license TEXT NOT NULL,
    repository_type TEXT NOT NULL,
    repository_url TEXT NOT NULL,
    FOREIGN KEY (data_id) REFERENCES datas(id)
);

CREATE TABLE keywords(
    id TEXT NOT NULL PRIMARY KEY,
    version_id TEXT NOT NULL,
    keyword TEXT NOT NULL,
    FOREIGN KEY (version_id) REFERENCES versions(id)
);

