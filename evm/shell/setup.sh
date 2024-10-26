#!/bin/bash

# Get the chainnames from the user
read -p "Enter the chainnames (separated by spaces): " -a CHAINNAMES

# Loop through each chainname and run the command
for CHAINNAME in "${CHAINNAMES[@]}"
do
  echo doing setup for "$CHAINNAME" 
  npx hardhat SETUP --network "$CHAINNAME"
done
