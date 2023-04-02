#!/bin/bash
## Creates automatically a new bevy plugin with Rust files to divide bevy concepts

input_name=$1
folder=$(echo "$input_name" | tr '[:upper:]' '[:lower:]')
struct="${input_name}Plugin"

echo "input_name = $input_name"
echo "folder = $folder"
echo "struct = $struct"

cd src
mkdir $folder
cd $folder

file_names=("components.rs" "logic_systems.rs" "depiction_systems.rs" "events.rs" "mod.rs" "resources.rs" "startup_systems.rs")

for file_name in ${file_names[@]}; do
    if [ -s $file_name ]; then
        echo "$folder/$file_name has content, doing nothing"
    else
        # The file is non-existing or empty.
        if [ "$file_name" == "mod.rs" ]; then
            echo -e "use bevy::prelude::*;\n" > $file_name

            echo -e "mod components;" >> $file_name
            echo -e "mod logic_systems;" >> $file_name
            echo -e "mod depiction_systems;" >> $file_name
            echo -e "mod events;" >> $file_name
            echo -e "mod resources;" >> $file_name
            echo -e "mod startup_systems;\n" >> $file_name

            echo -e "pub struct $struct;\n" >> $file_name

            echo -e "impl Plugin for $struct {" >> $file_name
            echo -e "    fn build(&self, app: &mut App) {" >> $file_name
            echo -e "        todo!()" >> $file_name
            echo -e "    }" >> $file_name
            echo -e "}" >> $file_name
        
        else
            echo -e "//use bevy::prelude::*;" > $file_name
        fi
    fi
done