<template>
    <router-view />
</template>

<script lang="ts" setup>
import { onMounted, computed } from 'vue';
import { useStore } from 'vuex';
import { changeLanguage } from './locale';
import { UserText } from './store';
import { getLocaleText } from './store/modules/user';

const store = useStore();

const locale = computed(() => store.getters[UserText + '/' + getLocaleText]);

// 设置语言 直接用 不用当成方法调用
onMounted(() => changeLanguage(locale.value));
</script>

<style lang="scss"></style>
