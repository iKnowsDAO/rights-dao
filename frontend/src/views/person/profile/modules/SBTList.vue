<template>
    <div class="sbt-list-container">
        <div class="sbt-badge" @click="dialogVisible=true">
            <img src="@/assets/images/sbt-test/sbt-bronze.png"/>
            <div>
                <span style="color: #FF7F3E;">{{exp}}</span>
                <span style="color: #999">/{{upLevelExp}}</span>
            </div>
        </div>
    </div>
    <SBTDialog v-if="dialogVisible"
               v-model:visible="dialogVisible"
               :achievementMenu="achievementMenu"
    />
</template>

<script lang="ts" setup>
    import { ref, onMounted, defineProps, PropType, defineEmits, computed } from 'vue';
    import { ElCard, ElIcon } from 'element-plus/es';
    import { Plus } from '@element-plus/icons-vue';
    import SBTDialog from '@/components/common/sbt/SBTDialog.vue';
    import { getUserAchievement } from "@/api/user";

    const props = defineProps({
        userPrincipal: {
            type: String,
            required: true,
        },
    });
    const achievementMenu = ref();
    const dialogVisible = ref(false);
    const exp = ref(240);
    const upLevelExp = ref(200);
    const level = ref(1);
    const missionForm = ref({});

    onMounted(() => {
        initAchievement();
    });

    const initAchievement = () => {
        getUserAchievement(props.userPrincipal).then(res => {
            console.log("getUserAchievement", res)
            console.log("getUserAchievement", JSON.stringify(res))
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
    }
    img {
        cursor: pointer;
        width: 50px;
        margin-top: 5px;
        margin-bottom: 5px;
    }
</style>
