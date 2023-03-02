key="key-test-storage-3"

key_bytes=`echo -n $key| xxd -p`
value_bytes=$(hexdump -e '16/1 "%02x"' /Users/Stefan/Desktop/316135046_234294325591048_8104861928528306_n.jpg)
value_bytes=`echo -n $value_bytes`
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "offchain_localStorageSet", "params": ["PERSISTENT", "'$key_bytes'", "'$value_bytes'"]}' http://localhost:9933/

