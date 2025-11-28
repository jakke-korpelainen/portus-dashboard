#!/bin/sh

# this script setups a fresh e.g. debian to autostart
# into chromium in kiosk mode running the compiled dashboard binary

# pre-requisite: autostart user exists

# CONFIG
INIT_DIR=$(pwd)
XINIT_CONFIG=/home/autostart/.xinitrc
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
apt install -y --no-install-recommends xcvt xserver-xorg-core x11-xserver-utils xinit chromium

echo "configuring autostart user"
rm ${XINIT_CONFIG}
# configure for any primary display
echo "setting up X11 configuration..."
cat <<EOL > ${XINIT_CONFIG}
#!/bin/sh
MODELINE=(cvt ${WIDTH} ${HEIGHT} ${REFRESH_RATE} | grep "Modeline" | sed -e 's/^.*"\(.*\)".*$/\1/')
DISPLAY=(xrandr -q | grep ' connected' | awk '{print $1}')
xrandr --newmode "${MODELINE}"
xrandr --addmode ${DISPLAY} ${WIDTH}x${HEIGHT}_${REFRESH_RATE}.00
xrandr --output ${DISPLAY} --mode ${WIDTH}x${HEIGHT}_${REFRESH_RATE}.00
chromium --kiosk --incognito '${DASHBOARD_URL}'
EOL

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

# Wait for user confirmation before rebooting
echo -n "reboot? (y/n): "
read REPLY
echo
if [ "$REPLY" = "y" ] || [ "$REPLY" = "Y" ]; then
    /sbin/reboot
fi
