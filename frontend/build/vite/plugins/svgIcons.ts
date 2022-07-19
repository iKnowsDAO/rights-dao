import SvgIconsPlugin from "vite-plugin-svg-icons"
import path from "path"

export function svgIconsPlugin(isBuild: boolean) {
    const svgIconsPlugin = SvgIconsPlugin({
        iconDirs: [path.resolve(process.cwd(), "src/assets/svg")],
        svgoOptions: isBuild,
        // default
        symbolId: "icon-[dir]-[name]",
    })
    return svgIconsPlugin
}
