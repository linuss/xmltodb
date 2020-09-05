DROP TABLE IF EXISTS artists_releases;
DROP TABLE IF EXISTS labels_releases;
DROP TABLE IF EXISTS extra_artists_releases;
DROP TABLE IF EXISTS formats_releases;
DROP TABLE IF EXISTS genres_releases;
DROP TABLE IF EXISTS styles_releases;
DROP TABLE IF EXISTS tracks_releases;
DROP TABLE IF EXISTS releases;

CREATE TABLE releases (
	id	int 	primary key,
	status	text,
	title	text,
	country	text,
	released text,
	notes	text,
	data_quality text,
	master_id int 	not null
);

CREATE TABLE artists_releases (
	release_id	int 	not null,
	artist_id 	int,
	name		text,
	role		text,	
	foreign key (release_id) references releases (id)
);


CREATE TABLE labels_releases (
	release_id	int not null,
	label_id	int,
	name		text,
	catno		text,
	foreign key (release_id) references releases (id)
);

CREATE TABLE extra_artists_releases (
	release_id 	int 	not null,
	artist_id	int,
	name		text,
	role		text,
	foreign key (release_id) references releases (id)
);

CREATE TABLE genres_releases (
	release_id	int 	not null,
	name		text 	not null,
	foreign key (release_id) references releases (id)
);

CREATE TABLE styles_releases (
	release_id	int 	not null,
	name		text 	not null,
	foreign key (release_id) references releases (id)
);
		
CREATE TABLE tracks_releases (
	release_id	int 	not null,
	position	text,
	title		text,
	duration	text,
	foreign key (release_id) references releases (id)
)
