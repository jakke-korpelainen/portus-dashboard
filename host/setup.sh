#!/bin/sh

# this script setups a fresh e.g. debian to autostart
# into chromium in kiosk mode running the compiled dashboard binary

# pre-requisite: autostart user exists

# CONFIG
INIT_DIR=$(pwd)
XINIT_CONFIG=/home/autostart/.xinitrc
AUTOSTART_BASH_CONFIG=/home/autostart/.bashrc
SYSTEMD_DASHBOARD=/etc/systemd/system/portus-dashboard.service
DASHBOARD_URL=http://localhost:3000
WIDTH=1080
HEIGHT=1920
REFRESH_RATE=60

# ensure root
echo "asserting root"
if [ "$(id -u)" -ne 0 ]; then
    su -c "$0 $@" root
    exit $?
fi

apt update

# Check if portus-dashboard is installed
echo "checking if binary is installed"
if ! command -v portus-dashboard &> /dev/null; then
    echo "portus-dashboard is not installed. Compiling from sources"
    apt install -y git curl build-essential
    cd /tmp
    git clone https://github.com/jakke-korpelainen/portus-dashboard.git
    cd portus-dashboard

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . $HOME/.cargo/env
    rustup target add x86_64-unknown-linux-musl
    cargo build --release
    install target/release/portus-dashboard /usr/local/bin
    cd ${INIT_DIR}
fi

echo "installing host packages"
apt update
apt install -y --no-install-recommends xcvt x11-xserver-utils xserver-common xserver-xorg xinit chromium

echo "configuring autostart user"
rm ${XINIT_CONFIG}
# configure for any primary display
echo "setting up X11 configuration..."
cat <<EOL > ${XINIT_CONFIG}
#!/bin/sh
xset s off
xset -dpms
xset s noblank
XRANDR_MODELINE=\$(cvt ${WIDTH} ${HEIGHT} ${REFRESH_RATE} | grep "Modeline" | sed -e 's/^.*Modeline //')
XRANDR_DISPLAY=\$(xrandr -q | grep ' connected' | awk '{print \$1}')
xrandr --newmode "\${XRANDR_MODELINE}"
xrandr --addmode \${XRANDR_DISPLAY} ${WIDTH}x${HEIGHT}_${REFRESH_RATE}.00
xrandr --output \${XRANDR_DISPLAY} --mode ${WIDTH}x${HEIGHT}_${REFRESH_RATE}.00
chromium --kiosk --incognito '${DASHBOARD_URL}'
EOL
chmod +x ${XINIT_CONFIG}

cat <<EOL > ${SYSTEMD_DASHBOARD}
[Unit]
Description=Portus Dashboard
After=network.target

[Service]
User=autostart
WorkingDirectory=/home/autostart
ExecStart=/usr/local/bin/portus-dashboard
Restart=always
RestartSec=5s
Environment=RUST_LOG=info
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=portus-dashboard

[Install]
WantedBy=multi-user.target
EOL

systemctl daemon-reload
systemctl enable portus-dashboard
systemctl start portus-dashboard
echo "dashboard systemd service created"

# setup autologin to tty1
mkdir -p /etc/systemd/system/getty@tty1.service.d
cat <<EOL > /etc/systemd/system/getty@tty1.service.d/override.conf
[Service]
ExecStart=
ExecStart=/sbin/agetty --autologin autostart --noclear %I $TERM
Type=idle
EOL

# setup startx tty1 to autostart bashrc
if ! grep -q "startx" ${AUTOSTART_BASH_CONFIG}; then
  echo 'if [ -z "$DISPLAY" ] && [ "$(tty)" = "/dev/tty1" ]; then' >> ${AUTOSTART_BASH_CONFIG}
  echo '  startx' >> ${AUTOSTART_BASH_CONFIG}
  echo 'fi' >> ${AUTOSTART_BASH_CONFIG}
fi

# wait for user confirmation before rebooting
echo -n "reboot? (y/n): "
read REPLY
echo
if [ "$REPLY" = "y" ] || [ "$REPLY" = "Y" ]; then
    /sbin/reboot
fi
