const {SourceMap} = require('./index.node');

for (let i = 0; i < 500000; i++) {
  let sm = new SourceMap();
}
