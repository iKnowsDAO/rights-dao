<template>
    <span class="delete-button" @click="dialogVisible=true">{{t('common.delete.title')}}</span>
    <el-dialog
        v-model="dialogVisible"
        :title="t('common.delete.title')"
        width="30%"
    >
        <span>{{t('common.delete.text')}}</span>
        <template #footer>
          <span class="dialog-footer">
            <el-button @click="dialogVisible = false">{{t('common.cancel')}}</el-button>
            <el-button type="primary" @click="confirmMethod()"
                       :loading="props.loading">{{t('common.confirm')}}</el-button>
          </span>
        </template>
    </el-dialog>
</template>

<script lang="ts" setup>
    import {ref, defineProps, onMounted, PropType} from 'vue';
    import {t} from '@/locale';
    import {ElButton, ElDialog} from 'element-plus/es';
    import {ApiResult} from "@/api/types";

    const props = defineProps({
        // 必要的内容，显示哪些
        loading: {
            type: Boolean,
            required: true,
        },
        id: {
            type: Number,
        },
        deleteFunction: {
            type: Function as PropType<(id?,callback?) => any>,
            required: true,
        },
    });
    const dialogVisible = ref(false);

    const confirmMethod = () => {
        //没有参数就不传
        props.deleteFunction(props.id ? props.id : null, (res) => {
            if (res.Ok) {
                //删除成功后的回调
                dialogVisible.value = false;
            }
        })
    }


</script>

<style lang="scss" scoped>
    .delete-button {
        color: rgb(133, 144, 166);
        &:hover {
            cursor: pointer;
        }
    }
</style>
