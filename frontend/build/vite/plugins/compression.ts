import type { Plugin } from "vite"
import compressionPlugin from "vite-plugin-compression"

export function viteCompressionPlugin(
    compress: "gzip" | "brotli" | "none" = "none",
    deleteOriginFile = false,
): Plugin | Plugin[] {
    const compressList = compress.split(",")

    const plugins: Plugin[] = []

    if (compressList.includes("gzip")) {
        plugins.push(
            compressionPlugin({
                ext: ".gz",
                deleteOriginFile,
            }),
        )
    }
    if (compressList.includes("brotli")) {
        plugins.push(
            compressionPlugin({
                ext: ".br",
                algorithm: "brotliCompress",
                deleteOriginFile,
            }),
        )
    }
    return plugins
}
