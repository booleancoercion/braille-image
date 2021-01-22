# braille-image
Binary rust crates that generates "ascii" art from images. (Not really ascii, since the braille characters are unicode)

For usage, run `braille-image --help`.

For example, running `braille-image -s 0.1 -i images/ferris_outline.png` produces:

⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⣀⠁⠁⠁⢀⣤⣄⠁⠁⠁⢀⡀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⢀⣠⣀⠁⠁⡾⠉⠻⣦⣠⡟⠁⠘⣧⣠⡾⠋⠹⡆⠁⢀⣠⣀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⡀⠁⠁⣾⠁⠈⠻⠾⠃⠁⠁⠈⠋⠁⠁⠁⠈⠋⠁⠁⠁⠻⠾⠋⠁⢹⡆⠁⢀⡀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⢠⡟⠉⠛⠶⠏⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠘⠷⠞⠋⠙⣧⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⣠⣤⣄⣸⡇⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⣟⣀⣤⣤⡀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⣏⠁⠁⠉⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠉⠁⠁⢈⡇⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⣠⣤⣤⣿⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⢸⣧⣤⣤⡀⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠸⡇⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⡿⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⣀⣻⡄⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⣼⣃⡀⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⣼⠋⠉⠉⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠈⠉⠉⢻⡄⠁⠁⠁⠁
⠁⠁⠁⠁⠘⢧⡀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠈⣿⣷⡄⠁⠁⠁⠁⢠⠁⠈⢻⣷⡄⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⣠⠟⠁⠁⠁⠁⠁
⠁⠁⠁⠁⣠⡾⠃⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⣧⣀⣠⣿⣿⣿⠁⠁⠁⠁⣿⣄⣠⣾⣿⣿⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠻⢦⣀⠁⠁⠁⠁
⠁⠁⣠⡾⠋⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠻⣿⣿⣿⣿⡟⠁⠁⠁⠁⠹⣿⣿⣿⣿⠟⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠉⠳⣤⡀⠁
⠁⢰⠋⠁⠁⠁⢀⡀⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠈⠙⠋⠁⠁⠁⠁⠁⠁⠁⠈⠉⠉⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⢀⠁⠁⣀⠁⠁⠁⢈⡗⠁
⠁⠘⢧⡀⠁⠁⠈⢿⣆⠁⠱⣦⣄⠁⠁⠁⠁⠁⠁⢀⠤⠒⠋⠉⠛⠲⢤⡀⠁⠁⠁⠁⢀⠴⠊⠉⠉⠉⠑⠠⡀⠁⠁⠁⠁⠁⠁⠁⣠⣾⡏⠁⣴⡏⠁⠁⢠⡞⠁⠁
⠁⠁⠁⠙⢦⡀⠁⠈⢿⣷⡄⠹⣿⢦⡀⠁⠁⠢⠄⠁⠁⠁⠁⠁⠁⠁⠁⠉⢢⡀⢀⡔⠁⠁⠁⠁⠁⠁⠁⠁⠈⠦⠖⠁⠁⠁⣠⡾⢫⡟⢀⣾⡿⠁⠁⣰⠏⠁⠁⠁
⠁⠁⠁⠁⠁⠙⢦⡀⠈⠻⣿⣦⡹⣆⠻⢦⣀⠁⠁⠁⠁⠁⠠⣤⣤⠤⠶⠶⠶⠟⠿⠖⠛⠛⠛⢒⣶⠆⠁⠁⠁⠁⢀⣠⡴⠞⠉⢀⡟⣰⠏⣼⠁⢀⡾⠃⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠙⢦⡀⠙⣮⠳⠿⠁⠁⠉⠛⢶⡀⠁⠁⠁⠈⠙⠳⠦⣤⣤⡀⣀⣀⣠⣤⠶⠛⠁⠁⠁⠁⢀⡴⠋⠁⠁⠁⠁⠸⠷⠋⣼⠃⣠⠟⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠙⢦⡘⣇⠁⠁⠁⠁⠁⠁⠙⢦⣄⠁⠁⠁⠁⠁⢀⡼⠁⠿⣍⡀⠁⠁⠁⠁⢀⣠⠶⠋⠁⠁⠁⠁⠁⠁⠁⠁⢰⡏⣴⠋⠁⠁⠁⠁⠁⠁⠁
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠈⠙⠋⠁⠁⠁⠁⠁⠁⠁⠁⠉⠛⠶⠦⠴⠞⠋⠁⠁⠁⠈⠙⠛⠛⠛⠛⠉⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠁⠈⠛⠁⠁⠁⠁⠁⠁⠁⠁⠁