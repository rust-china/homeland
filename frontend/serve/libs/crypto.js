
const crypto = require('crypto');

function md5Buffer(buffer) {
  return new Promise((res, rej) => {
    const hash = crypto.createHash('md5');
		hash.update(buffer)
		res(hash.digest('hex'));
  })
}

module.exports = {
	md5Buffer
}