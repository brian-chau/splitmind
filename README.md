# TUI Editor

## Dependencies
1. Install `screen`:
    ```
    sudo apt install screen
    ```
2. Install Rust:
    ```
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```
3. Install `nano`:
    ```
    sudo apt install nano
    ```

## How to use
1. Run the command:
    ```
    screen -c .screenrc
    ```
2. To switch to the next window,
    ```
    Ctrl + A, Ctrl + T
    ```
    * In other words, press and hold Ctrl, press A then release, then press T then release, all while holding Ctrl

3. To open a new file in `nano` with syntax highlighting:
    * `Ctrl+R`, then release both keys
    * `Alt+F`
    * Type the name of the file
    * Press Enter.

4. To save a file in `nano`:
    * Press `Ctrl+S`, then release.
    * Type `Y` then hit `Enter`

5. To close the file while keeping `nano` open on a blank unsaved file:
    * Press `Ctrl+S`, then release.
    * Type `Y` then hit `Enter`
    * Press `Ctrl+X` then release.
