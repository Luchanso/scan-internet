const net = require('net');
const debug = require('debug')('net');
const package = require('./package.json');
const PORT = 9090 || process.env.PORT;

debug('booting %o', package.name);

const server = net.createServer((socket) => {
    debug('client connected');

    socket.on('end', () => {
        debug('client disconnected');
    });

    socket.on('error', (err) => {
        debug('err %o', err);
    });

    socket.on('data', (buffer) => {
        debug('data', new Array(buffer).toString());
    })

    socket.write('hello\r\n');
    socket.pipe(socket);
    debug('sended key');
}).listen(PORT, () => {
    debug('listening on', PORT);
});

server.on('error', (err) => {
    debug('err %o', err);
})
