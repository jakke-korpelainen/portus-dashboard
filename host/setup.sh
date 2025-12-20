#!/bin/sh

# this script setups a fresh e.g. debian for wayland to autostart into
# firefox in kiosk mode running the compiled dashboard binary

# CONFIG
INIT_DIR=$(pwd)
SYSTEMD_DASHBOARD="/etc/systemd/system/portus-dashboard.service"
DASHBOARD_PORT=8080
DASHBOARD_REPO_URL=https://github.com/jakke-korpelainen/portus-dashboard
DASHBOARD_SECRETS_FILE="/etc/portus-dashboard/secrets"
KIOSK_USER="autostart"
KIOSK_USER_RUN="/run/user/$(id -u $KIOSK_USER)"
KIOSK_URL="http://localhost:${DASHBOARD_PORT}"
SWAY_LANG="fi"
SWAY_CONFIG="/home/${KIOSK_USER}/.config/sway/config"
SWAY_STARTUP="/home/${KIOSK_USER}/sway_startup.sh"
TARGET="aarch64-unknown-linux-gnu" # raspberry pi
# TARGET=x86_64-unknown-linux-musl # for regular debian etc
WIDTH=1920
HEIGHT=1080
REFRESH_RATE=60

# ensure root
echo "asserting root"
if [ "$(id -u)" -ne 0 ]; then
    su -c "$0 $@" root
    exit $?
fi

# try to locate from external media
EXT_SECRETS_FILE=$(find /media/* -name "portus-dashboard.secrets" 2>/dev/null | head -n 1)
if [ -z "$EXT_SECRETS_FILE" ]; then
    echo "portus-dashboard.secrets not found on external mounts (/media). Exiting."
    exit 1
fi

cat <<EOL > /etc/systemd/logind.conf
[Login]
HandleLidSwitch=ignore
IdleAction=ignore
EOL
systemctl restart systemd-logind

echo "installing host packages"
apt update
apt install -y --no-install-recommends curl sway firefox-esr xwayland kitty

echo "configuring ${KIOSK_USER} user"
# ensure kiosk user & permissions
adduser ${KIOSK_USER}
usermod -aG video,input,render ${KIOSK_USER}
mkdir -p "${KIOSK_USER_RUN}"
chown "${KIOSK_USER}" "${KIOSK_USER_RUN}"
chmod 0700 "${KIOSK_USER_RUN}"
mkdir -p "/home/${KIOSK_USER}/.config/sway"
cat <<EOL > ${SWAY_CONFIG}
output * bg #000000 solid_color
output HDMI-1 {
  res ${WIDTH}x${HEIGHT}
  transform 90
  pos ${WIDTH} 0
}
output DP-1 {
  res ${WIDTH}x${HEIGHT}
  transform 90
  pos ${WIDTH} 0
}
output Virtual-1 {
  res ${WIDTH}x${HEIGHT}
  transform 90
  pos ${WIDTH} 0
}
input * {
 xkb_layout "${SWAY_LANG}"
}
exec ${SWAY_STARTUP}
EOL

cat <<EOL > ${SWAY_STARTUP}
#!/bin/sh

# Function to check if the server is up and responds with 200 OK
check_server() {
  status_code=\$(curl -o /dev/null -s -w "%{http_code}\n" ${KIOSK_URL})
  if [ "\$status_code" -eq 200 ]; then
    return 0
  else
    return 1
  fi
}

# Wait until the server is up and responds with 200 OK
until check_server; do
  echo "Waiting for the dashboard service to be ready..."
  sleep 1
done

exec firefox -popups -chrome -kiosk -url "${KIOSK_URL}"
EOL
chmod +x ${SWAY_STARTUP}

# setup autologin to tty1
mkdir -p /etc/systemd/system/getty@tty1.service.d
cat <<EOL > /etc/systemd/system/getty@tty1.service.d/override.conf
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin ${KIOSK_USER} --noclear %I $TERM
Type=idle
EOL

if ! command -v portus-dashboard &> /dev/null; then
    echo "portus-dashboard is not installed. Downloading precompiled for ${TARGET}"
    cd /tmp
    # Replace the URL with the actual download link for the precompiled binary
    curl -L -o portus-dashboard ${DASHBOARD_REPO_URL}/releases/latest/download/portus-dashboard-${TARGET}
    chmod +x portus-dashboard
    install portus-dashboard /usr/local/bin
    cd ${INIT_DIR}
    echo "portus-dashboard installed successfully."
else
    echo "portus-dashboard is already installed."
fi

echo "creating systemd-creds for portus-dashboard"
mkdir -p /etc/portus-dashboard
systemd-creds encrypt ${EXT_SECRETS_FILE} ${DASHBOARD_SECRETS_FILE}
echo "portus-dashboard stored encrypted"

echo "creating systemd service for portus-dashboard"
cat <<EOL > ${SYSTEMD_DASHBOARD}
[Unit]
Description=Portus Dashboard
After=network.target

[Service]
User=${KIOSK_USER}
WorkingDirectory=/home/${KIOSK_USER}
ExecStart=/usr/local/bin/portus-dashboard
Restart=always
RestartSec=5s
Environment=RUST_LOG=info
Environment=PORT=${DASHBOARD_PORT}
LoadCredential=DIGITRANSIT_SUBSCRIPTION_KEY:${DASHBOARD_SECRETS_FILE}
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=portus-dashboard

[Install]
WantedBy=multi-user.target
EOL

systemctl daemon-reload
systemctl enable portus-dashboard
systemctl start portus-dashboard
echo "portus-dashboard systemd service complete"


cat <<EOL > /home/${KIOSK_USER}/.bash_profile
export XDG_SESSION_TYPE=wayland
export XDG_RUNTIME_DIR=${KIOSK_USER_RUN}
# exec sway
EOL

# wait for user confirmation before rebooting
echo -n "reboot? (y/n): "
read REPLY
echo
if [ "$REPLY" = "y" ] || [ "$REPLY" = "Y" ]; then
    /sbin/reboot
fi
