const addon = require('./index.node');
const fs = require('fs');
const express = require('express');

const app = express();
const port = 3000;

app.get('/img', (req, res) => {
    addon.rayon_image_gen({imgx: 1000, imgy: 1000}, function(buf) {
        fs.writeFileSync('data.jpg', buf, {encoding: 'binary'});
        res.setHeader('content-type', 'image/jpg');
        res.end(buf);
    });
})

app.listen(port, () => {
    console.log('server start');
});

