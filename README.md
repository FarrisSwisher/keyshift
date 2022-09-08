# keyshift
CLI tool to calculate change in semitones (pitch) after a change in bpm (tempo)

## About
When working with music, it is common that the producer/dj/etc will need to speed up or slow down a sound file. I've found that in some cases,
it can be difficult to predict the key of the now stretched sound file (if the user is allowing the key to change with tempo, that is).
So, I found the following equation online:

    Change_in_semitones = 12 x log2(new tempo / old tempo)

keyshift simply runs this equation and prints the output for the user.

## Use
keyshift expects the user to input two numbers: the old tempo (in bpm) first, then the new tempo (also in bpm)

from command line, enter:

    keyshift [old_tempo] [new_tempo] 

Assuming no errors, keyshift will return the pitch change in number of semitones. E.g., if the old tempo is 140 and the new is 280, the output
will be 12 because a doubled tempo will produce a key change of 12 semitones, or one full octave.
