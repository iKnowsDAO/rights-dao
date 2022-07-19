import Components from "unplugin-vue-components/vite"
import { ElementPlusResolver } from "unplugin-vue-components/resolvers"

export function unpluginVueComponents() {
    return Components({
        resolvers: [ElementPlusResolver()],
    })
}
