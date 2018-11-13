-- Authentication providers
CREATE TABLE authprovider (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    prov_name VARCHAR(20) NOT NULL
);

INSERT INTO authprovider(prov_name) VALUES('github');
INSERT INTO authprovider(prov_name) VALUES('gitlab');

-- Users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(30) NOT NULL,
    token TEXT NOT NULL,
    auth_provider INTEGER NOT NULL,
    ext_token TEXT NOT NULL,
    FOREIGN KEY(auth_provider) REFERENCES authprovider(id)
);