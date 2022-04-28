const WebSocket = require('ws');

ws = new WebSocket('wss://ws.kraken.com');

let subscribed = false;
ws.on('message', function message(data) {
  console.log('received: %s', data);

  if (subscribed == false) {
    ws.send('{"event":"subscribe", "subscription":{"name":"ticker"}, "pair":["XBT/USD"]}');

    subscribed = true;
  }
  
});


