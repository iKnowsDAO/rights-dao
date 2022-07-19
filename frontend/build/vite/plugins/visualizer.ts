/**
 * 包文件体积分析
 */
import visualizer from "rollup-plugin-visualizer"
import type { Plugin } from "vite"

export function visualizerPlugin() {
    if (process.env.REPORT) {
        return visualizer({
            filename: "./node_modules/.cache/visualizer/stats.html",
            open: true,
            gzipSize: true,
            brotliSize: true,
        }) as Plugin
    }
    return []
}
