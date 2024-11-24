# Rebels in the Sky


https://github.com/user-attachments/assets/aaa02f04-06db-4da5-8fa4-732b60083e66


It's the year 2101. Corporations have taken over the world.
The only way to be free is to join a pirate crew and start plundering the galaxy. The only means of survival is to play basketball.

Now it's your turn to go out there and make a name for yourself. Create your crew and start wandering the galaxy in search of worthy basketball opponents.

The game is under heavy development and breaking changes are often introduced. If you can't continue an old game because the save file is invalid, you probably need to start a new one or open an issue to check if the save file can be migrated.

[![Trailer on youtube](https://youtu.be/5Lu9MSgmTBc/0.jpg)](https://youtu.be/5Lu9MSgmTBc)

## Just try it out!

Connect via SSH to try the game.

`ssh rebels.frittura.org -p 3788`

Save files are deleted after 2 days of inactivity.

## Installation

### Build

There are some dependencies to build:

-   Linux: install `libasound2-dev` and `cmake` (for instance, on Ubuntu they can be installed with `sudo apt-get install -y libasound2-dev cmake`); 
-   MacOs: install `cmake` (for instance, using [brew](https://formulae.brew.sh/formula/cmake));
-   Windows: install [cmake](https://cmake.org/download/).

You need to have the [rust toolchain](https://www.rust-lang.org/tools/install). Then you can clone the repo and build the game with

`cargo build --release`

### With cargo

`cargo install rebels`

###  From the latest release page

- Download the latest release asset for your platform from https://rebels.frittura.org;
- Give execution permissions to the executable with `chmod +x rebels`

### Distro Packages

<details>
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/rebels-in-the-sky.svg)](https://repology.org/project/rebels-in-the-sky/versions)

</details>

#### Arch Linux

`rebels-in-the-sky` can be installed from the [official repositories](https://archlinux.org/packages/extra/x86_64/rebels-in-the-sky/):

```sh
pacman -S rebels-in-the-sky
```

## Run

This game runs as a terminal application, meaning that you just need to run the executable from your terminal with

`./rebels`

Suggested minimal terminal size: 160x48. Not all terminals support the game colors nicely, so you might need to try different ones. Here is a list of tested terminals:

-   Linux: whatever the default terminal is, it should work;
-   MacOs: [iTerm2](https://iterm2.com/), [tabby](https://tabby.sh/), [WezTerm](https://wezfurlong.org/wezterm/index.html);
-   Windows: [tabby](https://tabby.sh/).

**Important**: currently local bot teams are generated by default to make the game more enjoyable. This behaviour can be disabled by passing the `-f` flag to the executable. In the future, when more players will be available, the game will default to online teams only.

## Music

Previous versions had the option to play music directly in the game, but this was removed to reduce the binary size and now music is streamed from internet radios. Nevertheless, you can still listen to the game soundtrack directly by connecting to `https://radio.frittura.org/rebels.ogg`!

You can add more radio stations by including them in `assets/data/stream_data.json`. 


## Credits

- Planet gifs were generated using the [pixel planet generator](https://deep-fold.itch.io/pixel-planet-generator) by [Deep Fold](https://deep-fold.itch.io/).
- Special thanks to [Il Deposito](https://www.ildeposito.org) for inspiration and the great musical archive.

## Contribution

Join the [discord](https://discord.gg/ebjp33UrrV)! There is no fixed roadmap for the game yet, anyone is welcome to participate with ideas.

It is almost guaranteed that you will encounter bugs along your journey. If you do, please open an issue and describe what happened. If you are a developer and want to contribute, feel free to open a pull request.

## Running a relayer node

Running a relayer node helps keep the game decentralized. You can do so by running `rebels -n`. Other players can connect with your relayer at startup using `rebels -i <RELAYER-NODE-IP4-OR-IP6>`. If you do so, please consider opening a PR to add your address to the following list of known relayers.

### Additional relayer nodes

- `85.214.130.204`

## Running a SSH server

You can also run a SSH server with `rebels -j`.  Other players can connect with your server using `ssh <USERNAME>@<SERVER-IP4-OR-IP6> -p 3788`.

## License

This software is released under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html) license.
