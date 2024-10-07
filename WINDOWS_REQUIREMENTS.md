# Windows Installation Instructions

To set up your development environment on Windows, follow these steps:

### Step 1: Install CMake

1. **Download CMake**: Get the installer from the [CMake website](https://cmake.org/download/).
2. **Run the Installer**: Follow the prompts and make sure to add CMake to your system PATH.

### Step 2: Install MinGW-w64

1. **Download MinGW-w64**: Visit [this link](https://winlibs.com/) and pick the `Win64 UCRT Runtime` and make sure it's the `Win64`, I spent around 2 hours just to fix this simple issue if not more.
2. **Run the Installer**: Choose the **x86_64** architecture for 64-bit compatibility.
3. **Add MinGW-w64 to your PATH**: Ensure the MinGW-w64 `bin` directory is included in your system PATH.

### Step 3: Install Lua

1. **Download Lua**: Go to the [Lua website](https://www.lua.org/download.html) and download the latest Windows binaries.
2. **Set Up Lua**: Extract the files and add the Lua `bin` directory to your system PATH.

### Step 4: Install GOW

1. **Download GOW**: Get the installer from the [GOW GitHub repository](https://github.com/bmatzelle/gow/releases).
2. **Run the Installer**: Follow the prompts to install GOW, which provides Unix-like command-line tools for Windows.
3. **Add GOW to PATH**: Make sure the GOW installation directory is in your system PATH.

### Step 5: Install GNU pkg

1. **Install from MinGW**: GNU pkg is typically included with MinGW installations. Ensure it’s properly configured in your MinGW setup.

### Step 6: Verify Your Setup

After installing all the necessary tools, you can verify your installation by opening a Command Prompt and running:

```bash
cmake --version
gcc --version
g++ --version
lua -v
```

This will check if each tool is correctly installed and accessible.

If you encounter any issues or need further assistance, feel free to ask!