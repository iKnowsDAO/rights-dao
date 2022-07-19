/**
 * vite pwa 0 配置插件
 * https://github.com/antfu/vite-plugin-pwa
 */

import { VitePWA } from "vite-plugin-pwa"
import { ViteEnv } from "../../../types/model"

export function pwaPlugin(env: ViteEnv) {
    const {
        VITE_USE_PWA: shouldUsePwa,
        VITE_GLOB_APP_TITLE: appTitle,
        VITE_GLOB_APP_SHORT_NAME: shortName,
    } = env

    if (shouldUsePwa) {
        // vite-plugin-pwa
        const pwaPlugin = VitePWA({
            manifest: {
                name: appTitle,
                short_name: shortName,
                icons: [
                    // 图片还不知道怎么配置
                    // {
                    //     src: "./resource/images/pwa-192x192.png",
                    //     sizes: "192x192",
                    //     type: "image/png",
                    // },
                    // {
                    //     src: "./resource/images/pwa-512x512.png",
                    //     sizes: "512x512",
                    //     type: "image/png",
                    // },
                ],
            },
        })
        return pwaPlugin
    }
    return []
}
