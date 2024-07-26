#!/bin/zsh

workspace_dir="$HOME/workplace"
package_subdir="src"
setup_command="nvim"

projects_list=$(find -L "$HOME/workplace" -type d -maxdepth 1 -mindepth 1 -not -path "*/.*")

project_dir=$(echo $projects_list | fzf)

packages_list=$(find -L "$project_dir/$package_subdir" -type d -maxdepth 1 -mindepth 1 -not -path "*/.*")

package_dir=$(echo $packages_list | fzf)

eval $setup_command $package_dir

