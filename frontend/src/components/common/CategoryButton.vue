<template>
    <el-button class="category-button" type="primary">
        <span v-if="category.Tech!==undefined">{{t('post.help.category.tech')}}</span>
        <span v-else-if="category.Law!==undefined">{{t('post.help.category.law')}}</span>
        <span v-else-if="category.Safeguard!==undefined">{{t('post.help.category.safeguard')}}</span>
        <span v-else-if="category.Blacklist!==undefined">{{t('post.help.category.blacklist')}}</span>
        <span v-else>{{t('post.help.category.other')}}</span>
    </el-button>
</template>

<script lang="ts" setup>
    import { defineProps, onMounted } from 'vue';
    import {t} from '@/locale';
    import { ElButton } from 'element-plus/es';
    import { openTab } from '@/router/routers';

    const props = defineProps({
        // 必要的内容，显示哪些
        category: {
            type: Object,
            required: true,
        },
        // 是否可以点击（hover时鼠标变手）
        clickable: {
            type: Boolean,
            default: false,
        },
        clickCallback: Function,
    });


    const onClick = async () => {
        //可点击时执行方法
        if(props.clickable){
            try {
                // openTab('/person/profile/' + props.principalId)
            } catch (e) {
                console.error(e)
            }
        }
    }

    onMounted(() => {

    });

</script>

<style lang="scss">

</style>
