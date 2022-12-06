<template>
    <router-view />
</template>

<script lang="ts" setup>
import { onMounted, computed } from 'vue';
import { changeLanguage } from './locale';
import { useUserStore } from "@/store/user";

const userStore = useUserStore();
const locale = computed(() => userStore.getLocale);
// 设置语言 直接用 不用当成方法调用
onMounted(() => changeLanguage(locale.value));
</script>
