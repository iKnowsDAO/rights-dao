<template>
    <el-dialog
        v-model="visible"
        :before-close="onClose"
        :title="t('sbt.title')"
        :width="isPhone? '95%' : '35%'"
        destroy-on-close
        center
    >
        <el-table :data="tableData" style="width: 100%">
            <el-table-column prop="key" :label="t('sbt.achievement')"/>
            <el-table-column prop="target" :label="t('sbt.target')"/>
            <el-table-column prop="now" :label="t('sbt.achieved')"/>
            <el-table-column prop="progress" :label="t('sbt.progress')">
                <template #default="scope">
                    <el-progress type="circle" :percentage="80" width="50"/>
                </template>
            </el-table-column>
            <el-table-column prop="exp" :label="t('sbt.exp')"/>
            <el-table-column fixed="right" :label="t('sbt.operation')" width="120">
                <template #default>
                    <el-button link type="primary" size="small" @click="handleClick">Claim</el-button>
                </template>
            </el-table-column>
        </el-table>
        <template #footer>
          <span class="dialog-footer">
             <el-button @click="onClose">{{t('common.cancel')}}</el-button>
              <el-button type="primary" @click="submit()" :loading="loading">
                  {{t('common.confirm')}}
              </el-button>
          </span>
        </template>
    </el-dialog>
</template>
<script lang="ts" setup>
    import { ref, onMounted, defineProps, defineEmits, PropType } from 'vue';
    import { ElDialog, ElInput, ElButton, ElTable, ElTableColumn, ElProgress } from 'element-plus/es';
    import { t } from '@/locale';
    import { AchievementResult } from "@/api/types";


    const props = defineProps({
        visible: {
            type: Boolean,
            required: true,
            default: false
        },
        achievementMenu: {
            type: Object as PropType<AchievementResult>,
            required: true,
        }
    });
    const loading = ref(false);
    const emit = defineEmits(['update:visible'])
    //如果宽度小于769px则说明是平板以下的尺寸
    const isPhone = ref(document.documentElement.clientWidth < 769);
    const tableData = [
        {
            key: '获得一定量的回复',
            target: '10',
            now: '80',
            exp: '10',
            progress: '100%',
        },
        {
            key: '获得一定量的回复',
            target: '100',
            now: '80',
            exp: '30',
            progress: '80%',
        },
        {
            key: '2016-05-04',
            target: 'Tom',
            progress: 'No. 189, Grove St, Los Angeles',
        },
        {
            key: '2016-05-01',
            target: 'Tom',
            progress: 'No. 189, Grove St, Los Angeles',
        },
    ]

    const transferToTable = () => {

    }

    const onClose = () => {
        emit('update:visible');
    }

</script>
<style lang="scss">
</style>
