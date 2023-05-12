<template>
    <div class="sbt-list-container">
        <div class="sbt-badge">
            <SBTMedal v-if="level>0" :medalLevel="level" @click="dialogVisible=true"/>
            <el-icon v-else :size="30" @click="dialogVisible=true">
                <Medal/>
            </el-icon>
            <div>
                <span style="color: #FF7F3E;">{{exp}}</span>
                <span style="color: #999">/{{upLevelExp}}</span>
            </div>
            <el-button v-if="level > sbtLevel" @click="claimSBT()"
                       :loading="sbtLoading">Claim SBT
            </el-button>
        </div>
    </div>
    <SBTDialog v-model:visible="dialogVisible"
               :achievementMenu="achievementMenu"
    />
</template>

<script lang="ts" setup>
    import { ref, onMounted, defineProps } from 'vue';
    import { ElIcon, ElButton } from 'element-plus/es';
    import { Medal } from '@element-plus/icons-vue';
    import { t } from '@/locale';
    import SBTDialog from '@/components/common/sbt/SBTDialog.vue';
    import SBTMedal from '@/components/common/sbt/SBTMedal.vue';
    import { claimUserSBT, getUserAchievement, getUserSBTLevel } from "@/api/user";
    import { showMessageSuccess } from "@/utils/message";

    const props = defineProps({
        userPrincipal: {
            type: String,
            required: true,
        },
        sbtLevel: {
            type: Number,
            required: true,
        }
    });
    const achievementMenu = ref();
    const dialogVisible = ref(false);
    const sbtLoading = ref(false);
    const exp = ref(0);
    const upLevelExp = ref(10);
    const level = ref(0);

    onMounted(() => {
        initAchievement();
        getSBTLevel();
    });

    const claimSBT = () => {
        sbtLoading.value = true;
        claimUserSBT().then(res => {
            if (res.Ok) {
                showMessageSuccess(t('message.claim.sbt'))
            }
        }).finally(() => {
            sbtLoading.value = false;
        })
    }

    const getSBTLevel = () => {
        getUserSBTLevel(props.userPrincipal).then(res => {
            console.log("getUserSBTLevel", res)
            if (res.Ok) {
                exp.value = Number(res.Ok.experience);
                upLevelExp.value = Number(res.Ok.next_level);
                level.value = Number(res.Ok.level);
            }
        })
    }

    const initAchievement = () => {
        getUserAchievement(props.userPrincipal).then(res => {
            console.log("getUserAchievement", res)
            if (res.Ok) {
                achievementMenu.value = res.Ok;
            }
        })
    }


</script>

<style lang="scss" scoped>
    .sbt-list-container .sbt-badge {
        display: flex;
        flex-direction: column; /* 设置为纵向排列 */
        justify-content: center;
        align-items: center;
        margin-top: 5px;
        margin-bottom: 5px;
    }
    img {
        cursor: pointer;
        width: 50px;
    }
    .el-icon {
        cursor: pointer;
    }
</style>
