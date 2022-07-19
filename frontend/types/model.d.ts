export declare interface ViteEnv {
    VITE_TITLE:string;
    VITE_NETWORK: 'ic' | 'local';
    VITE_LOCAL_DFX: string; // 本地网络的话需要配置 dfx.json 文件位置
    VITE_CANISTER_IDS: string; // 这个应该是个数组，但是配置文件中不好读取数组，用逗号分隔
    VITE_CANISTER_APIS: string; // 同上 配置是字符串，实际按逗号分隔成数组

    VITE_KEYWORDS: string; // 首页关键词
    VITE_AUTHOR: string; // 首页作者
    VITE_DESCRIPTION: string; // 首页描述

    VITE_DROP_CONSOLE?: boolean; // 是否移除 console 输出
    VITE_DROP_DEBUGGER?: boolean; // 是否移除 debugger 点

    VITE_PORT: number;
    VITE_USE_MOCK: boolean;
    VITE_USE_PWA: boolean;
    VITE_PUBLIC_PATH: string;
    VITE_PROXY: [string, string][];
    VITE_GLOB_APP_TITLE: string;
    VITE_GLOB_APP_SHORT_NAME: string;
    VITE_USE_CDN: boolean;
    VITE_BUILD_COMPRESS: 'gzip' | 'brotli' | 'none';
    VITE_BUILD_COMPRESS_DELETE_ORIGIN_FILE: boolean;
    VITE_LEGACY?: boolean;
    VITE_USE_IMAGEMIN: boolean;
    VITE_GENERATE_UI: string;
    VITE_SHOW_DEBUG_SCRIPT: boolean;
    VITE_BLOG_PROXY_HOST: string;
}
