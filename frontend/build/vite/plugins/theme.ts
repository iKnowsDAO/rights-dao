/**
 * 网站主题颜色切换的Vite插件
 * https://github.com/anncwb/vite-plugin-theme
 */
import {
    viteThemePlugin as themePlugin,
    mixLighten,
    mixDarken,
    tinycolor,
} from "vite-plugin-theme"
// import { getThemeColors, generateColors } from "../../config/themeConfig"

export function viteThemePlugin() {
    // const colors = generateColors({
    //     mixDarken,
    //     mixLighten,
    //     tinycolor,
    // })

    const plugin = themePlugin({
        // 生成的很多个颜色方法
        colorVariables: [
            // ...getThemeColors(), ...colors
        ],
    })
    return plugin
}
