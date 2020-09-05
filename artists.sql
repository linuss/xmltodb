DROP TABLE IF EXISTS artist_aliases;
DROP TABLE IF EXISTS artist_group;
DROP TABLE IF EXISTS artist_urls;
DROP TABLE IF EXISTS artist_name_variations;
DROP TABLE IF EXISTS artists;

CREATE TABLE artists (
	id 		int	primary key,
	name		text	not null,
	realname 	text,
	profile		text,
	data_quality	text 	not null
);

CREATE TABLE artist_aliases (
	artist_id	int	not null,
	alias_id	int,
	name		text,	
	foreign key (artist_id) references artists (id)
);

CREATE TABLE artist_group (
	artist_id	int 	not null,
	group_id	int,
	name		text	,
	foreign key (artist_id) references artists (id)
);

CREATE TABLE artist_urls (
	url		text	not null,
	artist_id	int	not null,
	foreign key (artist_id) references artists (id)	
);

CREATE TABLE artist_name_variations (
	name_variation 	text 	not null,
	artist_id	int	not null,
	foreign key (artist_id) references artists (id)
);
	
