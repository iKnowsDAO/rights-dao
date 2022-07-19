import styleImport from "vite-plugin-style-import"

export function styleImportPlugin(isBuild: boolean) {
    if (!isBuild) return []
    const styleImportPlugin = styleImport({
        libs: [
            // 按需加载 element-plus
            {
                libraryName: "element-plus",
                esModule: true,
                ensureStyleFile: true,
                resolveStyle: (name) => {
                    const cssName: string = name.slice(3)
                    return `element-plus/packages/theme-chalk/src/${cssName}.scss`
                },
                resolveComponent: (name) => `element-plus/lib/${name}`,
            },
        ],
    })
    return styleImportPlugin
}
