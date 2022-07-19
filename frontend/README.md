# Right DAO
## 运行说明

后端代码位置 ../server
后端代码 2 种运行方式，本地环境 和 生产环境

1. 本地环境 在 server 目录下执行 dfx start，再执行 dfx deploy，每次存在代码更新需要再次dfx deploy。
将deploy获得的.dfx文件夹拷贝到frontend目录中，即可获得接口。
2. 生产环境 部署在 IC 网络的代码。

前端代码 2 种运行方式，dev 执行 和 部署方式

1. dev 执行 vite 的 development 模式，不输出代码可本地访问页面
2. 部署方式 vite 的 production 方式会生成正式代码在目录 dist/ 下 a.本地 nginx 代理执行 b. 部署至 ic 线上

注意: 打包生产环境代码会执行 vue-tsc 检查 typescript 代码，因此原先的 @ 和 canisters/xxx 等应当解析路径的引入，必须配置 tsconfig 的 paths 参数
手动配置，否则无法打包生产环境。

命令说明

```sh
# 开发模式  vite开发 和 本地后端
# 访问 http://localhost:3000
# 配置 canister id -> ../services/.dfx/local/canister_ids.json
# 配置 canister api -> ../services/.dfx/local/canisters/
# 配置 network -> local
npm run dev
```

```sh
# 开发前端模式  vite开发 和 线上后端 使用时谨慎处理数据修改
# 访问 http://localhost:3000
# 配置 canister id -> ../services/canister_ids.json
# 配置 canister api -> ../services/.dfx/local/canisters/
# 配置 network -> ic
npm run dev_frontend
```

```sh
# alpha模式  vite生产 nginx 和 本地后端
# 访问 http://localhost:8080 这个看 nginx 是怎么代理 dist 目录下的代码了
# 配置 canister id -> ../services/.dfx/local/canister_ids.json
# 配置 canister api -> ../services/.dfx/local/canisters/
# 配置 network -> local
npm run alpha
# nginx 代理打包文件进行访问是 需要把 /api 前缀的请求代理到本地后端运行端口
# nginx 需要类似如下配置 并将其他类型请求均代理至 dist 目录
# location ~ ^/api { proxy_pass http://127.0.0.1:8000; }
```

```sh
# beta模式  vite生产 nginx 和 线上后端 使用时谨慎处理数据修改
# 访问 http://localhost:8080 这个看 nginx 是怎么代理 dist 目录下的代码了
# 配置 canister id -> ../services/canister_ids.json
# 配置 canister api -> ../services/.dfx/local/canisters/
# 配置 network -> ic
npm run beta
# nginx 代理打包文件进行访问是 需要把 /api 前缀的请求代理到线上运行地址
# nginx 需要类似如下配置 并将其他类型请求均代理至 dist 目录
# location ~ ^/api { proxy_pass https://mainnet.dfinity.network; }
```

```sh
# 生产模式  vite生产 线上 和 线上后端
# 访问 canister_id.ic0.app
# 配置 canister id -> ../services/canister_ids.json
# 配置 canister api -> ../services/.dfx/local/canisters/
# 配置 network -> ic
# npm run build # 该命令生成生产代码，并不部署
# 部署至线上，需要支付 cycles 该命令会包含 npm run build
dfx deploy --network=ic --with-cycles=200000000000 assets # 第一次部署携带 cycles  assets 是 frontend 里前端代码在 dfx 项目中的名字
dfx deploy --network=ic assets # 或者不携带 cycles
```

```sh
# 报告模式  vite 查看代码依赖情况
npm run report # 本地即可看到代码依赖统计
```

```sh
# 其他说明
# 由于 vite build 不输出配置日志，经常出现待不到需要的文件，提示的错误又莫名其妙
# 因此使用 vite 的 dev 模式运行但是读取对应 mode 的配置，看看到底哪些文件到不到了
npm run alpha-check
npm run beta-check
npm run build-check
```
