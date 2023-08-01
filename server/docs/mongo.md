# 构造 mongo 环境

## 创建用户

```js
// create root
use admin
db.createUser({user:"root", pwd:"root#starn",roles:[{role:"root",db:"admin"}]})
db.createUser({user:"starn", pwd:"starn#website",roles:[{role:"readWrite",db:"starn"}]})
```