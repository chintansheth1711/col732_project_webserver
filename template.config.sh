ip a add {{ip}}/24 dev eth0
ip link set eth0 up

dropbear -E > log.txt
adduser -D {{username}}
echo '{{username}}:{{password}}' | chpasswd
