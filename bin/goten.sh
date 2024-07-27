#!/bin/zsh

workspace_dir="$HOME/workplace"
package_subdir="src"
editor_command="nvim"

projects_list=$(find -L "$HOME/workplace" -type d -maxdepth 1 -mindepth 1 -not -path "*/.*")

project_dir=$(echo $projects_list | fzf)

packages_list=$(find -L "$project_dir/$package_subdir" -type d -maxdepth 1 -mindepth 1 -not -path "*/.*")

package_dir=$(echo $packages_list | fzf)

session_name=$(basename $package_dir)

tmux new-session -d -c $package_dir -s "$session_name-editor" "$editor_command $package_dir"
tmux new-session -d -c $package_dir -s "$session_name-build"
tmux attach-session -t "$session_name-editor"

