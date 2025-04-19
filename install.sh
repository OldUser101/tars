#!/bin/bash

# Installs tars for the current running user

cd ~
git clone https://github.com/OldUser101/tars.git
mkdir -p ~/.tars/plugins
cp ~/tars/tars.py ~/.tars/
ln -s ~/.tars/tars.py ~/.tars/tars
echo 'export PATH="$PATH:$HOME/.tars"' >> ~/.bashrc
source ~/.bashrc
rm -rf ~/tars