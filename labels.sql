/*
<?xml version="1.0" encoding="UTF-8"?>
<label>
   <images>
      <image type="primary" uri="" uri150="" width="600" height="600" />
      <image type="secondary" uri="" uri150="" width="600" height="600" />
   </images>
   <id>13</id>
   <name>Glasgow Underground</name>
   <contactinfo>contact@glasgowunderground.com</contactinfo>
   <profile>Glasgow Underground was set up in 1997 by [a=Kevin McKay]. He began the label releasing his own music and collaborations with friends and other Glasgow-based artists ([url=https://www.discogs.com/artist/2046-16B?filter_anv=1&amp;anv=Omid+16B]Omid 16B[/url], [a441], [a447], [a4054]). Through his DJing and work for various music magazines (Muzik, Jockey Slut, Mixmag Update) he met some of his favourite producers of the time ([a2583], [a77], [a170413], et al) and convinced some of them to join him releasing on GU. &#xD;
&#xD;
In the early 2000s, Kevin met [a=Mylo] and set up the [l=Breastfed Recordings] label. The global success of the "[m53063]" release, meant Glasgow Underground was put on hold. After leaving the Breastfed label in 2011, Kevin re-started Glasgow Underground and since then the label has gone on to re-invigorate Romanthony's catalogue with remixes from the likes of [a4492], [a970905], [a3221], [a1095], [a408185], [a2432212], [a506574], [a2605338], [a3513068], [a50620], [a52501], and [a2293268]. &#xD;
&#xD;
The label has also introduced producers like [a2836171], [a1137042], [a1921483], and [a4482463] as well has releasing established and rising stars such as [a1547806], [a1254764], and [a1100914].</profile>
   <data_quality>Correct</data_quality>
   <urls>
      <url>http://www.glasgowunderground.com</url>
      <url>http://www.facebook.com/glasgowunderground</url>
      <url>http://www.instagram.com/glasgowunderground</url>
      <url>http://soundcloud.com/glasgowunderground</url>
      <url>http://twitter.com/kevinmckay</url>
      <url>http://www.youtube.com/glasgowunderground</url>
   </urls>
   <sublabels>
      <label id="318209">Glasgow Underground Music</label>
      <label id="282060">Glasgow Underground Recordings</label>
      <label id="4200">Muzique Tropique</label>
   </sublabels>
</label>
*/

DROP TABLE IF EXISTS label_urls;
DROP TABLE IF EXISTS labels;

CREATE TABLE labels (
	id		int 	primary key,
	name		text 	not null,
	contact_info	text		,
	profile		text		,
	data_quality	text	not null,
	parent_label	int
);

CREATE TABLE label_urls (
	id		serial	primary key,
	url		text 	not null,
	label_id	int	not null,	
	foreign key (label_id)
		references labels (id)
);
		

