# 构造 mongo 环境

## 登录 mongo container

```shell
docker ps
docker exec -it ${mongo-container-id} /bin/bash
mongosh "mongodb://starn:statics%40web@mongo:27017/starn"
```

## 创建用户

```js
// create user
use admin
db.createUser({user:"root", pwd:"root#starn",roles:[{role:"root",db:"admin"}]})
use starn
db.createUser({user:"starn", pwd:"statics@web",roles:[{role:"readWrite",db:"starn"}]})
```

## 测试环境启动 mongo

```sh
docker run -d -p 27017:27017 --name dev_mongo -v /Users/ogres/Desktop/data:/data/db mongo
```
