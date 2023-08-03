# 构造 mongo 环境

## 登录 mongo container

```shell
docker ps
docker exec -it ${mongo-container-id} /bin/bash
mongo --port 20217 -u starn -p starn#website
```

## 创建用户

```js
// create root
use admin
db.createUser({user:"root", pwd:"root#starn",roles:[{role:"root",db:"admin"}]})
db.createUser({user:"starn", pwd:"starn#website",roles:[{role:"readWrite",db:"starn"}]})
```
