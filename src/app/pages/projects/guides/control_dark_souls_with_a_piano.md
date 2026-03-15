---
title: "Control Dark Souls with a piano"
date: 2020-07-08
lastmod: 2021-03-08
tags:
- Video Games
- Controller
- Dark Souls
- Piano
- Midi Keyboard
- Python
- C api
- DirectX
- Regex
- Windows
---
## Controlling Dark Souls with a Piano? But Why?

So i've had this midi piano keyboard for a while now but I don't use it all that much. This project was intended to give me an excuse to use the keyboard more, with the hope being that it could help make me better at hitting specific notes or maybe even CHORDS. It didn't quite pan out that way as I played for like half an hour and then gave up but it's the thought that counts!

## How did I make it work?

So after some googling and a few dead ends I found this [youtube video by BenOS](https://www.youtube.com/watch?v=R-pcY65_HDg)(no relation) where he did exactly what I was looking to do, and gave a brief explanation to boot! The core solution he used is actually pretty simple. You run a single Python script which connects to the Midi piano (or any other Midi device) for input, converts specific notes into Keypresses, and then injects these keypresses into the Operating system. It even reads in a config file specifying which Midi notes should be converted into which Keypresses!

The specifics of his implementation are as follows:

- Connect to Midi Devices using pygame's midi interface
- Output chosen Keypresses using pynput's keyboard interface

My initial implementation followed this to the letter, with most of the code being copied directly from the youtube video (couldn't find a Github link or anything so yes I hand typed it). It was a bit of a pain to implement but it seemed to be working! Hitting keys on the Piano successfully typed into notepad as if I were typing on my keyboard, the Dark Souls settings menu even let me set the keyboard controls using the Piano. Of course, it had to fail at the last hurdle. When actually in game the Piano inputs did nothing. Zip. Nada. Diddly Squat. So I did some more debugging and found that in Dark Souls 3 the Piano could NOT set the controls in the settings. Interesting...

So I went searching for alternative ways to inject keypresses into Windows from Python, ways which might actually WORK in games. From that digging I found [this](https://pythonprogramming.net/direct-input-game-python-plays-gta-v/) webpage which provided example code for controlling games from Python! The new control system uses a Python interface to a DirectX C api.

I still don't fully understand this DirectX api, but I do know that there was only one significant change that I needed to make to get it to work. Previously I could inject a keypress of the letter "a" by calling a function with the letter "a" itself as an argument. Unfortunately with DirectX that won't fly. DirectX uses a series of KeyCodes to identify each key on a standard keyboard. So to inject a keypress of the "a" key, you need to use that key's key code (0x1E). This isn't particularly intuitive to work with so I tracked down the C header that lists ALL THE KEYCODES ([dinput.h](https://github.com/glfw/glfw/blob/master/deps/mingw/dinput.h)) and extracted all the keycodes and their names into a Python....array? dictionary? something like that....some collection that lets me index by keyname and get the relevant key code out.(it's not important or useful info, but I did this by copying the entire file into something like Notepad++ or Atom, and using Regex find and replace to first delete all lines that don't contain key codes, then to modify all keycode lines to swap the C #define formatting to Python array formatting. If you haven't used regex much I'd HIGHLY recommend looking into it as it makes modifying large amounts of text muuuuuch faster)

Anyway, after all that I now can inject a keypress of the "A" key by passing DIK_A to a function rather than needing to remember 0x1E. Nice. Finally I just needed to update the config file that maps Midi notes onto key presses, aaaaand time to try Dark Souls again....

## Results

BAM! It WORKED! I was controlling the chosen undead like....well like someone with a REALLY messed up controller...but I was controlling them nonetheless! Not only that, but with this script I could potentially hook up ANY Midi instrument to play ANY game! (as long as said game doesn't REQUIRE analogue controls...might need to do some further modifications to get THAT working)

If you are interested in looking at the code, or just want to download and run it blindly then click [here](https://github.com/Isienmai/MidiPianoToKeyboard/). The config file can be found in the profiles directory. Also on looking through it seems I didn't comment it...like at ALL (BAD Ben)...but hopefully it shouldn't be tooo obtuse.

Thanks for reading. Enjoy!
