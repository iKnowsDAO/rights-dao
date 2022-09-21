<template>
    <span class="dao-state open" v-if="state.Open!==undefined">{{t('dao.state.open')}}</span>
    <span class="dao-state accepted" v-else-if="state.Accepted!==undefined">{{t('dao.state.accepted')}}</span>
    <span class="dao-state rejected" v-else-if="state.Rejected!==undefined">{{t('dao.state.rejected')}}</span>
    <span class="dao-state executing" v-else-if="state.Executing!==undefined">{{t('dao.state.executing')}}</span>
    <span class="dao-state succeeded" v-else-if="state.Succeeded!==undefined">{{t('dao.state.succeeded')}}</span>
    <span class="dao-state failed" v-else-if="state.Failed!==undefined">{{t('dao.state.failed')}}</span>
</template>

<script lang="ts" setup>
    import { defineProps, PropType } from 'vue';
    import {t} from '@/locale';
    import {ApiDaoState} from "@/api/types";

    const props = defineProps({
        state: {
            type: Object as PropType<ApiDaoState>,
            required: true,
        },
    });

</script>

<style lang="scss" scoped>
    /*状态的样式*/
    .dao-state {
        padding: 2px 12px;
        border-radius: 14px;
        color: white;
        white-space: nowrap;
        &.open, &.accepted, &.executing {
            background-color: rgb(103, 194, 58);
        }
        &.succeeded {
            background-color: #059669;
        }
        &.rejected {
            background-color: rgb(144, 147, 153);
        }
        &.failed {
            background-color: rgb(245, 108, 108);
        }
    }
</style>
