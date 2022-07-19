import vitePluginImp from "vite-plugin-imp"

export function impPlugin() {
    return vitePluginImp({
        libList: [
            {
                libName: "element-plus",
                style: (name) => {
                    return `element-plus/theme-chalk/${name}.css`
                },
            },
        ],
    })
}
