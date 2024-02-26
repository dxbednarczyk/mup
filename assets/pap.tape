# VHS documentation
#
# Output:
#   Output <path>.gif               Create a GIF output at the given <path>
#   Output <path>.mp4               Create an MP4 output at the given <path>
#   Output <path>.webm              Create a WebM output at the given <path>
#
# Require:
#   Require <string>                Ensure a program is on the $PATH to proceed
#
# Settings:
#   Set FontSize <number>           Set the font size of the terminal
#   Set FontFamily <string>         Set the font family of the terminal
#   Set Height <number>             Set the height of the terminal
#   Set Width <number>              Set the width of the terminal
#   Set LetterSpacing <float>       Set the font letter spacing (tracking)
#   Set LineHeight <float>          Set the font line height
#   Set LoopOffset <float>%         Set the starting frame offset for the GIF loop
#   Set Theme <json|string>         Set the theme of the terminal
#   Set Padding <number>            Set the padding of the terminal
#   Set Framerate <number>          Set the framerate of the recording
#   Set PlaybackSpeed <float>       Set the playback speed of the recording
#   Set MarginFill <file|#000000>   Set the file or color the margin will be filled with.
#   Set Margin <number>             Set the size of the margin. Has no effect if MarginFill isn't set.
#   Set BorderRadius <number>       Set terminal border radius, in pixels.
#   Set WindowBar <string>          Set window bar type. (one of: Rings, RingsRight, Colorful, ColorfulRight)
#   Set WindowBarSize <number>      Set window bar size, in pixels. Default is 40.
#   Set TypingSpeed <time>          Set the typing speed of the terminal. Default is 50ms.
#
# Sleep:
#   Sleep <time>                    Sleep for a set amount of <time> in seconds
#
# Type:
#   Type[@<time>] "<characters>"    Type <characters> into the terminal with a
#                                   <time> delay between each character
#
# Keys:
#   Escape[@<time>] [number]        Press the Escape key
#   Backspace[@<time>] [number]     Press the Backspace key
#   Delete[@<time>] [number]        Press the Delete key
#   Insert[@<time>] [number]        Press the Insert key
#   Down[@<time>] [number]          Press the Down key
#   Enter[@<time>] [number]         Press the Enter key
#   Space[@<time>] [number]         Press the Space key
#   Tab[@<time>] [number]           Press the Tab key
#   Left[@<time>] [number]          Press the Left Arrow key
#   Right[@<time>] [number]         Press the Right Arrow key
#   Up[@<time>] [number]            Press the Up Arrow key
#   Down[@<time>] [number]          Press the Down Arrow key
#   PageUp[@<time>] [number]        Press the Page Up key
#   PageDown[@<time>] [number]      Press the Page Down key
#   Ctrl+<key>                      Press the Control key + <key> (e.g. Ctrl+C)
#
# Display:
#   Hide                            Hide the subsequent commands from the output
#   Show                            Show the subsequent commands in the output

Output pap.mp4

Require pap

Set Shell "bash"
Set FontSize 18
Set Width 1200
Set Height 600

Type "# Here's how to set up a Minecraft server from scratch using pap!"
Enter
Sleep 3s

Type "mkdir my-server && cd my-server"
Enter
Sleep 1s

Type "# First, let's initialize the server:" 
Enter
Sleep 1s

Type "pap server init --minecraft-version 1.20.4 --loader fabric"
Enter
Sleep 5s

Type "# Then, add the plugins / mods you want:" 
Enter
Sleep 1s

Type "pap project add fabric-api"
Enter
Sleep 1s

Type "# You can also specify specific version IDs."
Enter
Sleep 1s

Type "pap project add chunky --version-id=t8SbUchF"
Enter
Sleep 1s

Type "# pap signs the EULA for you. Your server is ready to go!"
Enter
Sleep 1s

Type "java -jar fabric.jar"
Sleep 5s

Ctrl+C
Ctrl+L

Type "# If you already have a server and want to move it to a different machine, pap can do that!"
Enter
Sleep 1s

Type "# Let's use the server we just set up as an example:"
Enter
Sleep 1s

Type "mkdir another-server && cd another-server"
Enter
Sleep 1s

Type "cp ../pap.lock ."
Enter
Sleep 1s

Type "# To see what pap is doing under the hood, use the --verbose flag"
Enter
Sleep 1s

Type "pap --verbose server install"
Enter
Sleep 5s

Type "# pap can also update existing dependencies.
Enter
Sleep 1s

Type "pap project update"
Enter
Sleep 5s

Type "# Seems like pap did its job, let's check:"
Enter
Sleep 1s

Type "tree"
Enter
Sleep 5s