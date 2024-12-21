#!/bin/sh

# Based on https://docs.kurtosis.com/install/#ii-install-the-cli
echo "deb [trusted=yes] https://apt.fury.io/kurtosis-tech/ /" | sudo tee /etc/apt/sources.list.d/kurtosis.list
sudo apt update
sudo apt install kurtosis-cli