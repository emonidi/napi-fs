const {ensureDir} = require(".");
ensureDir("./testdir/ts/tasda").then(res=>{
    console.log(res)
});