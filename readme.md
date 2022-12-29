Tentative discord commands:

search [track/album/playlist] term
	Returns the first 10 results for the term. Can be specified to return either albums, tracks, or playlists. Default behavior returns tracks. 

queue [track/album/playlist] term
	Adds a track to the queue. Those with the DJ role can add all of the songs of an album or playlist to the queue.

skip [--force]
    Adds a vote to skip the current song. Three votes are required to skip the song. Those with the DJ role can use --force to skip immediately. 

Only those with the DJ discord role will be able to use the following commands:

play [track/album/playlist] term
	Immediately plays a song, album, or playlist. This will bypass the queue, and when playing an album or playlist will delete the contents of the queue.

set setting
	queue bool
		Enables/disbales the queue command.
	shuffle bool
		Enabels/disables spotify shuffle.
	repeat bool
		Enables/disables spotify repeat.
