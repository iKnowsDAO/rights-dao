import { defineConfig, loadEnv, UserConfig } from 'vite';
import path from 'path';
import fs from 'fs';

import { createVitePlugins } from './build/vite/plugins';
import { ViteEnv } from './types/model';

enum ConfigMode {
    development = 1, // 防止 0 情况 if 出错
    dev_frontend,
    alpha,
    beta,
    production,
}

// 输出配置文件
export default defineConfig(({ command, mode }) => {
    console.log('command ->', command);
    console.log('mode ->', mode);

    const configMode = getConfigMode(mode); // 获取配置模式
    console.log('config mode ->', ConfigMode[configMode]); // 输出查询出来的配置模式

    const readEnv = loadEnv(mode, './env');
    // @ts-ignore force transform, not a bit problem for string variable
    const viteEnv: ViteEnv = readEnv; // 导入设置的环境变量，会根据选择的 mode 选择文件
    // but matters other types
    if (readEnv.VITE_DROP_CONSOLE !== undefined)
        viteEnv.VITE_DROP_CONSOLE = readEnv.VITE_DROP_CONSOLE === 'true';
    if (readEnv.VITE_DROP_DEBUGGER !== undefined)
        viteEnv.VITE_DROP_DEBUGGER = readEnv.VITE_DROP_DEBUGGER === 'true';
    console.log('viteEnv ->', viteEnv); // 输出加载的变量

    const network = getNetwork(viteEnv);
    console.log('network ->', network);

    const canisterIds = getCanisterIds(viteEnv);
    const canisterApis = getCanisterApis(viteEnv);

    const canistersAlias = initAlias(canisterIds, network, canisterApis);

    const location = getLocation(viteEnv);
    console.log('server location ->', location); //

    process.env.configMode = ConfigMode[configMode];
    process.env.network = network;
    process.env.location = location;

    if (configMode !== ConfigMode.production) {
        // 开发模式总是有个 warning 不想看到
        canistersAlias['vue-i18n'] = 'vue-i18n/dist/vue-i18n.cjs.js';
    }
    mode = getMode(configMode);
    const isBuild = mode === 'production';
    const common: UserConfig = {
        publicDir: 'public', // 该目录下文件会原封不动存放至 dist
        mode, // 运行模式
        define: {
            'process.env.NODE_ENV': JSON.stringify(getNodeEnv(configMode)), // 接口文件里面需要用来判断 莫名其妙要加双引号
            'process.env': process.env, // 环境变量
        },
        plugins: [...createVitePlugins(viteEnv, isBuild)], // 插件
        resolve: {
            alias: {
                ...canistersAlias, // canister 接口文件位置的映射
                '@': path.resolve(__dirname, 'src'), // @符号要解析
                '~/': `${path.resolve(__dirname, 'src')}/`, // element-plus 可能要用
                // "vue-i18n": "vue-i18n/dist/vue-i18n.cjs.js", // 浏览器总是有 warning，这样就不显示了
            },
            extensions: ['.js', '.ts', '.jsx', '.tsx', '.vue'], // import 可以省略的拓展名
        },
        build: {
            minify: isBuild ? 'esbuild' : false, // 默认为 Esbuild，它比 terser 快 20-40 倍，压缩率只差 1%-2%
            terserOptions: {
                compress: {
                    // 线上环境移除console
                    // drop_console:
                    //     configMode == ConfigMode.production
                    //         ? true // 线上部署的生产打包一定不包含
                    //         : viteEnv.VITE_DROP_CONSOLE === undefined
                    //         ? isBuild
                    //         : viteEnv.VITE_DROP_CONSOLE, // 生产环境去除 console
                    // drop_debugger:
                    //     configMode == ConfigMode.production
                    //         ? true // 线上部署的生产打包一定不包含
                    //         : viteEnv.VITE_DROP_DEBUGGER === undefined
                    //         ? isBuild
                    //         : viteEnv.VITE_DROP_DEBUGGER, // 生产环境去除 debugger
                },
            },
            rollupOptions: {
                // external: ["element-plus"],
                output: {
                    manualChunks: {
                        // 每个 '键' 都表示一个分包块，'值' 包含列出的模块及其所有依赖项
                        vue: ['vue', 'vue-router', 'pinia'], // 目前打包还是这个最小，还没有 bug
                        'element-plus': ['element-plus'],
                    },
                    // manualChunks(id) {
                    //     if (id.includes("node_modules")) {
                    //         return "vendor"
                    //     }
                    //     // if (
                    //     //     id.includes("node_modules") &&
                    //     //     id.match(/element-plus|legacy/)
                    //     // ) { // TODO 本来是想解决打包过大问题，但是现在发现，打的包会失效无法线上运行
                    //     //     return id
                    //     //         .toString()
                    //     //         .split("node_modules/")[1]
                    //     //         .split("/")[0]
                    //     //         .toString()
                    //     // }
                    // },
                },
            },
        },
        css: {
            preprocessorOptions: {
                scss: {
                    charset: false, // TODO 没作用 element-plus 的 index.css 在被打包时会报错 warning: "@charset" must be the first rule in the f
                    // additionalData: `@use "~/assets/theme/element-variables.scss" as *;`,
                },
            },
        },
        envDir: 'env',
        clearScreen: false,
    };

    console.log(`process.env.NODE_ENV -> ${common.define['process.env.NODE_ENV']}`);

    if (!isBuild) {
        return {
            // serve 独有配置 开发模式
            ...common,
            server: {
                proxy: {
                    '/api': {
                        target: location,
                        changeOrigin: true,
                        rewrite: (path) => path,
                    },
                },
                cors: true,
                host: '0.0.0.0',
            },
        };
    } else {
        return {
            // build 独有配置 生产模式
            ...common,
        };
    }
});

// 判断配置模式
function getConfigMode(mode: string): ConfigMode {
    if (ConfigMode[mode]) {
        return ConfigMode[mode];
    }
    throw new Error('can not recognize mode: ' + mode);
}

// 判断网络
function getNetwork(viteEnv: ViteEnv) {
    if (!viteEnv.VITE_NETWORK) {
        throw new Error('config network is missing. please set config VITE_NETWORK');
    }
    const dfxNetwork = process.env.DFX_NETWORK;
    if (dfxNetwork && dfxNetwork !== viteEnv.VITE_NETWORK) {
        console.log(
            `config process.env.DFX_NETWORK is ${dfxNetwork}. but VITE_NETWORK is ${viteEnv.VITE_NETWORK}`,
        );
    }
    return viteEnv.VITE_NETWORK;
}

// 获取后端运行地址
function getLocation(viteEnv: ViteEnv): string {
    const position = viteEnv.VITE_LOCAL_DFX;
    if (position) {
        const dfxJson = require(position);
        return 'http://' + dfxJson.networks.local.bind;
    }
    return 'https://mainnet.dfinity.network';
}

// 根据环境参数加载 canister 名称 和 id 之间的关系，主要关联需要用到的 canister_ids.json 文件 可以多个
function getCanisterIds(viteEnv: ViteEnv) {
    // 找出 2 段字符串数组的交集
    const intersect = function (a: string[], b: string[]): string[] {
        const result: string[] = [];
        for (const t in a) if (b[t]) result.push(t);
        return result;
    };

    const positions = viteEnv.VITE_CANISTER_IDS ? viteEnv.VITE_CANISTER_IDS.split(',') : [];
    const canisterIds = {};
    try {
        for (const position of positions) {
            if (fs.existsSync(position)) {
                const read = JSON.parse(fs.readFileSync(position).toString());
                if (!Object.keys(read).length)
                    console.log('canister ids file is not empty:', position);
                const inter = intersect(Object.keys(read), Object.keys(canisterIds));
                for (const name in inter)
                    console.log(
                        `cover canister id ${name} from ${canisterIds[name]} to ${read[name]}`,
                    );
                Object.assign(canisterIds, read);
            } else {
                console.error('canister ids file is not exist', position);
            }
        }
    } catch (e) {
        console.error('read canister ids failed. the path is', positions);
    }
    return canisterIds;
}

// 根据环境参数加载 canister 名称 和 接口 之间的关系，主要关联需要用到的 canisters 文件夹 可以多个，dfx 编译出的结果
// 调用其他 canister 都要有接口文件才好
function getCanisterApis(viteEnv: ViteEnv) {
    const positions = viteEnv.VITE_CANISTER_APIS ? viteEnv.VITE_CANISTER_APIS.split(',') : [];

    const canisterApis = {};
    try {
        for (const position of positions) {
            const dirs = fs.readdirSync(position);
            console.log('dirs: ' + position + ' ->', dirs);
            for (const dir of dirs) {
                if (fs.lstatSync(position + '/' + dir).isDirectory()) {
                    const value = position + '/' + dir;
                    if (canisterApis[dir]) {
                        console.log(
                            `cover canister api ${dir} from ${canisterApis[dir]} to ${value}`,
                        );
                    }
                    canisterApis[dir] = value;
                }
            }
        }
    } catch (e) {
        console.error('read canister api failed. the path is ', positions);
    }

    return canisterApis;
}

// 总的设置别名 程序里需要用到的地方有 2 种
// 1. 是 canister 的部署 id，要调用方法，有 id 才知道去哪里调用
// 2. 是 canister 的接口文件，有接口文件，才知道哪些方法能用哪些不能用
// 3. 按道理还有个运行网络的问题，但是目前默认，线上网络是 ic ，其他都是 local 本地环境
function initAlias(canisterIds: {}, network: string, apiPositions: {}) {
    const canistersAlias = {};
    for (const canister in canisterIds) {
        // 这里将 id 设置到 process.env 对象里面，标准接口文件中有用到
        const key = canister.toUpperCase() + '_CANISTER_ID';
        process.env[key] = canisterIds[canister][network];
        console.log(key, canisterIds[canister][network]);

        if (apiPositions[canister]) {
            // 接口文件的引入因为是动态配置，这里统一进行映射，代码里写 key ，会在真正运行时候 resolve 成实际的位置
            canistersAlias['canisters/' + canister] = path.join(
                __dirname,
                apiPositions[canister] + '/index.js',
            );
            console.log(
                'canisters/' + canister,
                path.join(__dirname, apiPositions[canister] + '/index.js'),
            );
        } else {
            console.log(`canister ${canister} api position is missing.`);
        }
    }
    return canistersAlias;
}

function getMode(configMode: ConfigMode) {
    let mode = '';
    switch (configMode) {
        case ConfigMode.development:
            mode = 'development';
            break;

        case ConfigMode.dev_frontend:
            mode = 'development'; // 前端是开发模式
            break;

        case ConfigMode.alpha:
            mode = 'production';
            break;

        case ConfigMode.beta:
            mode = 'production';
            break;

        case ConfigMode.production:
            mode = 'production';
            break;

        default:
            throw new Error(`what a config config mode: ${configMode} ${ConfigMode[configMode]}`);
    }
    return mode;
}

function getNodeEnv(mode: ConfigMode): string {
    let env = '';
    switch (mode) {
        case ConfigMode.development:
            env = 'development';
            break;

        case ConfigMode.dev_frontend:
            env = 'production'; // 后端用的是 ic
            break;

        case ConfigMode.alpha:
            env = 'development';
            break;

        case ConfigMode.beta:
            env = 'production'; // 后端用的是 ic
            break;

        case ConfigMode.production:
            env = 'production'; // 后端用的是 ic
            break;

        default:
            throw new Error(`what a config config mode: ${mode} ${ConfigMode[mode]}`);
    }
    return env;
}
