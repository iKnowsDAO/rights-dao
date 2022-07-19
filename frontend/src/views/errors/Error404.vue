<template>
    <div class="error-404-container">
        <div class="tip">{{t('message.error.notFound')}}</div>
        <button @click="goBack(router)">{{t('common.back')}}</button>
    </div>
</template>

<script lang="ts" setup>
    import {t} from '@/locale';
    import {goBack} from "@/router/routers";
    import {useRouter} from "vue-router";

    const router = useRouter();

</script>

<style lang="scss">
    .error-404-container {
        text-align: center;
    }
</style>
