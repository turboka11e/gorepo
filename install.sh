#!/bin/bash

# Define a function to create a directory with user permission
create_directory_with_permission() {
    local directory="$1"

    if [ -d "$directory" ]; then
        echo "Directory '$directory' already exists."
    else
        read -p "Directory '$directory' does not exist. Create it? (y/n): " create_permission
        if [ "$create_permission" = "y" ] || [ "$create_permission" = "Y" ]; then
            if mkdir -p "$directory"; then
                echo "Directory '$directory' created successfully."
            else
                echo "Failed to create directory '$directory'."
            fi
        else
            echo "Directory '$directory' was not created."
        fi
    fi
}

echo "### Installing gorepo ###"

echo "## Build Binary"
cargo build --release

echo "## Copy Rust-Binary"
create_directory_with_permission "$HOME/.local/share/bash-completion/gorepo"
cp ./target/release/gorepo ~/.local/share/bash-completion/gorepo/gorepo-completion -v

echo "## Copy bash-completion script"
create_directory_with_permission "$HOME/.local/share/bash-completion/completions"
cp ./scripts/bash-completion/gorepo ~/.local/share/bash-completion/completions/gorepo -v

echo "## Copy gorepo"
create_directory_with_permission "$HOME/.local/bin"
cp ./scripts/gorepo ~/.local/bin/gorepo -v

# Define the alias and its command
alias_name="gorepo"
alias_command=". ~/.local/bin/gorepo"

# Check if the alias is already defined in ~/.bashrc
if ! grep -q "alias $alias_name=" ~/.bashrc; then
    # Append the alias to ~/.bashrc
    echo "alias $alias_name=\"$alias_command\"" >> ~/.bashrc
    echo "Alias added to ~/.bashrc."
else
    echo "Alias $alias_name is already defined in ~/.bashrc."
fi
