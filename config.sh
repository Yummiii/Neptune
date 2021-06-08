#!/bin/bash
wallpaper="https://danbooru.donmai.us/data/original/7d/6e/__gabriel_tenma_white_gabriel_dropout_drawn_by_yuki_kokoro__7d6e7137fe16d61acfa2fc97feff4208.png"
avatar="https://i.imgur.com/6lxpyPY.png"
flameshot_cfg="https://firebasestorage.googleapis.com/v0/b/nepnep-98c6a.appspot.com/o/flameshot.ini?alt=media&token=971dca01-4757-49dc-9a76-907307765a84"
hotkeys="https://firebasestorage.googleapis.com/v0/b/nepnep-98c6a.appspot.com/o/hotkeys.dconf?alt=media&token=7ef59c16-10bd-4fc6-ad11-486603550b2a"

echo "Configurações do usuario"
sudo hostnamectl set-hostname "Isla"
gsettings set org.gnome.desktop.interface gtk-theme "Adwaita-dark"
gsettings set org.gnome.desktop.interface clock-format "12h"
gsettings set org.gnome.desktop.wm.preferences button-layout "appmenu:minimize,close"
sudo wget "$avatar" -O "/var/lib/AccountsService/icons/$USERNAME"
[ ! -d "$HOME/.local/share/backgrounds/" ] && mkdir "$HOME/.local/share/backgrounds/"
wget "$wallpaper" -O "$HOME/.local/share/backgrounds/$(basename "$wallpaper")"
gsettings set org.gnome.desktop.background picture-uri "$HOME/.local/share/backgrounds/$(basename "$wallpaper")"
git config --global user.name "Yummiii"
git config --global user.email "Leeo05050@outlook.com"
clear

echo "DNFs"
sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'
sudo dnf remove firefox gnome-boxes -y
sudo dnf upgrade -y
sudo dnf install sqlitebrowser kitty firewall-config gnome-tweaks playerctl flameshot chrome-gnome-shell code gnome-extensions-app openssl -y
clear

echo "Flatpaks"
sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install flathub org.mozilla.firefox com.github.tchx84.Flatseal org.filezillaproject.Filezilla org.kde.krita com.discordapp.Discord com.github.micahflee.torbrowser-launcher org.telegram.desktop com.transmissionbt.Transmission com.spotify.Client org.videolan.VLC org.gnome.Boxes com.jetbrains.DataGrip org.gnome.Builder -y
clear

echo "Node"
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash
source ~/.bashrc
nvm install --lts
clear

echo "Rust"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc
cargo install viu
clear

echo "Misc"
if [ -d "$HOME/.config/monitors.xml" ]; then
    sudo cp -v "$HOME/.config/monitors.xml" "/var/lib/gdm/.config/"
    sudo chown gdm:gdm "/var/lib/gdm/.config/monitors.xml"
fi
wget "$flameshot_cfg" -O "$HOME/.config/flameshot/flameshot.ini"
git clone https://github.com/Yummiii/Neptune.git "$HOME/.nepnep"
chmod +x "$HOME/.nepnep/print.sh"
cd "$HOME/.nepnep"
cargo build --release
sudo ln -s "$HOME/.nepnep/target/release/neptune" "/usr/bin/neptune"
clear

echo "Hotkeys"
wget "$hotkeys" -O "$HOME/hotkeys.dconf"
dconf load "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/" < "$HOME/hotkeys.dconf"
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/', '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom1/','/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom2/','/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom3/','/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom4/','/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom5/','/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom6/','/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom7/','/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom8/']"
rm "$HOME/hotkeys.dconf"
clear

echo "Extensões"
pip3 install --user git+https://github.com/essembeh/gnome-extensions-cli
gnome-extensions-cli install 615 1319 779 36 750 1276