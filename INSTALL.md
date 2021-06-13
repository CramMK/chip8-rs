# Install

`sdl2` together with its dependencies (`sdl_gfx sdl2_image sdl2_mixer sdl2_ttf`)
are required to be installed on your machine.

## Linux (Arch-Based)
Arch-based distros can use these packages:
`yay -Syu sdl2 sdl2_gfx sdl2_image sdl2_mixer sdl2_ttf`

## Linux (Debian-Based)
0. Ensure your packages are up-to-date.
1. Grab and extract the latest release from this repository.
2. Open a terminal and install the latest version of rustup using `curl` here: https://rustup.rs/
3. In the terminal enter the following commands:
    `sudo apt install build-essential`
    `sudo apt-get install libsdl2-dev`
    `sudo apt-get install libsdl2-image-dev`
    `sudo apt-get install libsdl2-mixer-dev`
    `sudo apt-get install libsdl2-ttf-dev`
    `sudo apt-get install libsdl2-gfx-dev`
You can now use the emulator by opening a terminal navigating to your extracted folder and using the command `./Chip8-KS`.
You might need to allow the file to be run as a program if you get an error.
  
## Windows
0. Make sure you have an installation of Visual Studio with the English language pack!
1. Install Rustup from the official website https://www.rust-lang.org/tools/install
2. Download the following `sdl2` dependencies:
    1. https://www.libsdl.org/release/SDL2-devel-2.0.14-VC.zip
    2. https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.5-VC.zip 
    3. https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-devel-2.0.4-VC.zip
    4. https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-devel-2.0.15-VC.zip
3. Unpack the downloaded files.
4. Copy all the files from the `lib` directory of the respective extracted folder to `C:\user\%USERNAME%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`.
6. `sdl2_gfx` needs to be compiled for windows manually. Download and Install GitBash https://git-scm.com/downloads
7. Create an empty folder, right-click in it and select "Git Bash Here".
8. Copy and Paste the following command and press Enter `git clone https://github.com/microsoft/vcpkg`. Wait for the download to complete.
9. Close Git Bash and open the new "vcpkg" folder. Run `bootstrap-vcpkg.bat` as administrator. If done correctly a "vcpkg.exe" file should appear.
10. Shift-right-click and select "Open PowerShell Window Here" in the "vcpkg" folder. A command prompt will open.
11. Copy and Paste the following command and press Enter `vcpkg.exe install sdl2-gfx --triplet x64-windows`(It'll give you an error if you're missing VS). 
12. Navigate to \installed\x64-windows\lib in your vcpkg folder and copy `SDL2.lib` and `SDL2_gfx.lib` to `C:\user\%USERNAME%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`.
13. Grab the latest release from this repository.

You can now use the emulator with the .exe launcher!

