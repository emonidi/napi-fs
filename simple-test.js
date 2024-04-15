const { ensureDir, emptyDir } = require('.')
ensureDir('./testdir/ts/tasda').then((res) => {
  console.log(res)
})

emptyDir('./testdir').then(res=>{
    console.log(res);
})
