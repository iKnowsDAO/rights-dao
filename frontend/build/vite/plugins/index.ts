import type { Plugin } from 'vite';

import { ViteEnv } from '../../../types/model';

import vue from '@vitejs/plugin-vue';
import { viteHtmlPlugins } from './html';
import { svgIconsPlugin } from './svgIcons';
import { styleImportPlugin } from './styleImport';
import { viteCompressionPlugin } from './compression';
// import PurgeIcons from 'vite-plugin-purge-icons';
import { visualizerPlugin } from './visualizer';
import { viteThemePlugin } from './theme';
import { imageminPlugin } from './imagemin';
import { pwaPlugin } from './pwa';
import { unpluginVueComponents } from './unpluginVueComponents';
import { legacyPlugin } from './legacy';

export function createVitePlugins(viteEnv: ViteEnv, isBuild: boolean) {
    const {
        VITE_BUILD_COMPRESS: compressType,
        VITE_BUILD_COMPRESS_DELETE_ORIGIN_FILE: shouldBuildCompressDeleteFile,
        VITE_USE_IMAGEMIN: shouldUseImagemin,
    } = viteEnv;

    const vitePlugins: (Plugin | Plugin[])[] = [];

    vitePlugins.push(vue());
    vitePlugins.push(...viteHtmlPlugins(viteEnv, isBuild)); // 注入配置字符串
    vitePlugins.push(svgIconsPlugin(isBuild)); // svg 图标资源整合管理
    vitePlugins.push(styleImportPlugin(isBuild)); // 样式导入
    // vitePlugins.push(PurgeIcons({})); // 图片管理
    vitePlugins.push(visualizerPlugin()); // 可视化依赖分析，非生产模式
    // vitePlugins.push(viteThemePlugin()) // 主题
    vitePlugins.push(legacyPlugin()); // 低级浏览器支持
    // vitePlugins.push(impPlugin()) // TODO 有 bug 导致 rollup 不能打包
    // vitePlugins.push(unpluginVueComponents()) // TODO 按照官方的方法导入的，也不行

    if (isBuild) {
        // 生成模式执行
        vitePlugins.push(
            viteCompressionPlugin(compressType, shouldBuildCompressDeleteFile), // 压缩
        );
        shouldUseImagemin && vitePlugins.push(imageminPlugin()); // 压缩图片 vite-plugin-imagemin 貌似这个插件不支持 m1 芯片 以后再检查吧
        vitePlugins.push(pwaPlugin(viteEnv)); // 缓存应用什么的
    }

    return vitePlugins;
}
