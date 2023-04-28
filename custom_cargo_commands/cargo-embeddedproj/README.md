# Cargo command for setting up openocd enviroment

Cargo command for setting up the configuration files for an openocd working enviroment for embedded programming. Some of the files might need some changes to fit your needs. I have only tested it on Windows 11, so I do not know if it will work for other operating systems.

## Setup
1. Clone the repository
2. Move the "config_files" folder to a destination of your choice
3. Add this path to an enviroment variable called "EMBED_RUST"
4. Move the .exe file to your ../.cargo/bin
5. You can now delete the config files in the cargo-embeddedproj repository
6. Open a Rust project and use "cargo embeddedproj"

Feel free to send me tips for improvements.

Hope you find the command useful.