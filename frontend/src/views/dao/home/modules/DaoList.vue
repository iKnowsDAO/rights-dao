<template>
    <div class="dao-list-container">
        <button @click="router.push('/dao/submit')">add</button>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, computed} from 'vue';
    import {t} from '@/locale';
    import {useRoute, useRouter} from 'vue-router';

    const router = useRouter();


</script>
<style lang="scss">
    .dao-list-container {}
</style>
