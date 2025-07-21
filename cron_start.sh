#!/bin/sh
echo "Setting up cron job..."
echo "$CRON_SCHEDULE" > /etc/crontabs/root
/usr/sbin/crond -f
sleep 60
