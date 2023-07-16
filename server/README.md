# server

## 用户权限管理机制

- action: 行为, 可执行的操作 read, write
- role: 角色, 一个角色包含若干个 action(actions列表), 角色可执行自己所有的 action
- catalog: 目录, 用于标记页面路径, 通常有两层 catalog & page
- user: 用有密码, 登录时做密码校验. 用户有 roles 数组 [{catalog: "catalog.*/page", action: "read/write"}]

## 用户登录

user + passwd 登录, 密码校验通过就校验登录成功. md5(user)为用户的 uid. 用户检索使用 uid 进行检索.
