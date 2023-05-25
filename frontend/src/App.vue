<template>
    <Connect2ICProvider :client="client">
        <router-view/>
    </Connect2ICProvider>
</template>

<script lang="ts" setup>
    import { onMounted, computed } from 'vue';
    import { changeLanguage } from './locale';
    import { useUserStore } from "@/stores/user";
    import { createClient } from "@connect2ic/core"
    import { PlugWallet } from "@connect2ic/core/providers/plug-wallet"
    import { AstroX } from "@connect2ic/core/providers/astrox"
    import { Connect2ICProvider } from "@connect2ic/vue"
    import { getSBTInfo } from "@/api/user";

    const userStore = useUserStore();
    const locale = computed(() => userStore.getLocale);
    onMounted(() => {
        // 设置语言 直接用 不用当成方法调用
        changeLanguage(locale.value)
        //提前加载勋章列表数据
        getSBTInfo()
    });
    const client = createClient({
        providers: [
            new AstroX(),
            // new PlugWallet(),
        ]
    })
</script>
