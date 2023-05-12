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
            <el-table-column prop="key" :label="t('sbt.achievement.title')" width="120"/>
            <el-table-column prop="target" :label="t('sbt.target')"/>
            <el-table-column prop="now" :label="t('sbt.achieved')"/>
            <el-table-column prop="progress" :label="t('sbt.progress')">
                <template #default="scope">
                    <el-progress type="circle" :percentage="scope.row.progress"
                                 :width="50"/>
                </template>
            </el-table-column>
            <el-table-column prop="exp" :label="t('sbt.exp')"/>
        </el-table>
        <template #footer>
          <span class="dialog-footer">
             <el-button @click="onClose">{{t('common.cancel')}}</el-button>
              <el-button type="primary" @click="claimAll()" :loading="loading">
                  Claim All
              </el-button>
          </span>
        </template>
    </el-dialog>
</template>
<script lang="ts" setup>
    import { ref, reactive, onMounted, defineProps, defineEmits, PropType, watch } from 'vue';
    import { ElDialog, ElInput, ElButton, ElTable, ElTableColumn, ElProgress } from 'element-plus/es';
    import { t } from '@/locale';
    import { AchievementResult } from "@/api/types";
    import { calculatePercent } from "@/utils/math";
    import { SumDivision } from "@/api/bounty";
    import { claimUserAchievement } from "@/api/user";
    import { showMessageSuccess } from "@/utils/message";


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
    const tableData = ref([
        {
            key: t('sbt.achievement.active_user'),
            target: 10,
            now: 0,
            exp: 10,
            progress: 0,
        },
        {
            key: t('sbt.achievement.post_comment'),
            target: 10,
            now: 0,
            exp: 10,
            progress: 0,
        },
        {
            key: t('sbt.achievement.reputation'),
            target: 100,
            now: 0,
            exp: 10,
            progress: 0,
        },
        {
            key: t('sbt.achievement.issued_bounty'),
            target: 1,
            now: 0,
            exp: 20,
            progress: 0,
        },
        {
            key: t('sbt.achievement.received_bounty'),
            target: 1,
            now: 0,
            exp: 20,
            progress: 0,
        },
    ])

    const transferToTable = (menu: AchievementResult) => {
        const array = [];
        for (const key in menu) {
            if (menu.hasOwnProperty(key)) {
                const value = menu[key];
                transICP(value);
                const item = {
                    key: t('sbt.achievement.' + key),
                    target: Number(value.target),
                    now: Number(value.completion),
                    exp: Number(value.experience),
                    progress: calculatePercent(value.completion, value.target),
                };
                //@ts-ignore
                array.push(item);
            }
        }
        tableData.value = array;
        console.log("tableData", tableData.value)
    }

    const transICP = (item) => {
        switch (item.keyword) {
            case "issued bounty":
            case "received bounty":
                item.completion = SumDivision(item.completion)
                item.target = SumDivision(item.target)
                return
        }
        return
    }

    const claimAll = () => {
        loading.value = true;
        claimUserAchievement().then(res => {
            if (res.Ok) {
                console.log("res", res)
                showMessageSuccess(t('message.claim.success'))
            }
        }).finally(() => {
            loading.value = false;
        })
    }

    onMounted(() => {
        if (props.achievementMenu) {
            transferToTable(props.achievementMenu)
        }
    })

    watch(
        () => props.achievementMenu,
        (nv) => {
            transferToTable(props.achievementMenu);
        },
    );

    const onClose = () => {
        emit('update:visible');
    }

</script>
<style lang="scss">
</style>
