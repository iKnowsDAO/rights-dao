import legacy from "@vitejs/plugin-legacy"

// Vite默认的浏览器支持基线是原生ESM。本插件为不支持原生ESM的传统浏览器提供支持
// 为最终bundle中的每个chunk生成一个相应的legacy chunk，用@babel/reset-env进行转换，并以SystemJS模块的形式发布（仍然支持代码分割！）
export function legacyPlugin() {
    return legacy({
        targets: ["ie >= 11"],
        additionalLegacyPolyfills: ["regenerator-runtime/runtime"],
    })
}
