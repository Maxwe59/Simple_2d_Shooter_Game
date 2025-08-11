A simple top-down 2d shooter game created in bevy.

# Game functionalities:

  ### Player functionalities:
  >basic player movement using AWSD keybinds

  >basic player rotation so that the player always faces the user's mouse

  >player hand entities that are children of the body

  >toggle between "fist" mode, and "rifle" mode, by pressing 2 to equip the rifle

  ### Map functionalities:
  > Defined map boundaries that reject player from going outside of undefined boundaries
  > partitioned map into defined amount of grids to avoid rendering large rectangle for frostum culling

  ### Rifle functionalities:
  > configurable rifle struct with various stats that can be altered (including bullet spread, fire rate, bullet speed, bullet size, rifle length, ect)
  > "trail" effect that simulates bullet drag between spawned bullets, using child circle entities to simulate drag

Game Showcase:
![App demo](https://media.discordapp.net/attachments/1202105605310119987/1404370189138530304/image.png?ex=689af129&is=68999fa9&hm=5f26ba17cdda7eeeb4ae0cb34539da60d43628faada3572ffe87f048fe20ffc0&=&format=webp&quality=lossless&width=1570&height=920)
