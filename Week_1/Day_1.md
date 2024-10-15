# Installing Rust on Windows
To install Rust on a Windows OS, follow these step-by-step instructions:

## Step 1: Download the Rust Installer
1. Go to the official Rust installation page: (https://www.rust-lang.org/tools/install).
2. Choose the appropriate installer for your system architecture (32-bit or 64-bit) and download it.

## Step 2: Run the Installer
1. Locate the downloaded file (usually named `rustup-init.exe`).
2. Run the installer. This will open a command prompt window.
3. Follow the on-screen instructions.
  
## Step 3: Verify the Installation
1. After the installation completes, open a new command prompt window.
2. Type the following command to check if Rust is installed correctly:
   ```bash
   rustc --version
   ```
   This command should display the version of the Rust compiler you installed. If you see an error or no output, ensure that your PATH environment variable is set correctly.

## Additional Recommendations
- **Install Visual Studio Code**: While it's not mandatory, using an IDE like Visual Studio Code can enhance your development experience with features like syntax highlighting and debugging support. You can install it from [Visual Studio Code's official site](https://code.visualstudio.com/) and also install the **Visual Studio Build Tools**.
- **Update Rust**: To ensure you have the latest version of Rust, periodically run:
   ```bash
   rustup update
 
