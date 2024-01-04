#!/bin/bash
## This script copies the icons from the tabler-icons package to the assets folder
##
## This script is designed to simplify the process of copying icons from the tabler-icons
## package to the assets folder. We don't need to copy all the icons, just the ones we
## use.
##
## Tabular-icons is licensed under the MIT license. https://tabler.io/icons

## Declare the paths
TABULAR_PATH="./node_modules/@tabler/icons-png/icons"
DESTINATION="./assets/ui/icons"

## If the folder doesn't exist, create it by calling pnpm install
if [[ ! -d "${TABULAR_PATH}" ]]; then
	pnpm install
fi

## If the destination folder doesn't exist, create it (sanity check)
if [[ ! -d "${DESTINATION}" ]]; then
  echo -n "Creating destination folder"
	mkdir -p "${DESTINATION}"
fi

## Declare the icons we want to copy
declare -a arr=(
	"arrow-badge-right.png"
	"settings.png"
	"door-exit.png"
)

## Loop through the icons and copy them to the destination folder
for i in "${arr[@]}"; do
	cp "${TABULAR_PATH}/${i}" "${DESTINATION}/${i}"
done
