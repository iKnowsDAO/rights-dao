<template>
    <div class="dao-detail-information-container">
        <div class="container">
            <el-card>
                <template #header>
                    <div class="header">
                        <h4><b>{{t('dao.information.title')}}</b></h4>
                    </div>
                </template>
                <div class="dao-form">
                    <div>
                        <b>{{t('dao.information.action.label')}}</b>
                        <span>{{t('dao.information.action.add')}}</span>
                    </div>
                    <div v-if="proposal.payload.execute_args.AddGovernanceMember.id">
                        <b>{{t('dao.information.target')}}</b>
                        <div class="user">
                            <el-icon><UserFilled /></el-icon>
                            <Username :principalId="proposal.payload.execute_args.AddGovernanceMember.id.toString()"
                                      :username="targetUser!==undefined && targetUser.name!==''
                                                      ? targetUser.name: ''"
                                      :clickable="true"/>
                        </div>
                    </div>
                    <div>
                        <b>{{t('dao.information.start')}}</b>
                        <span>{{formatDateToMinutes(proposal.created_at)}}</span>
                    </div>
                    <div>
                        <b>{{t('dao.information.end')}}</b>
                        <span>{{formatDateToMinutes(proposal.payload.execute_args.AddGovernanceMember.deadline)}}</span>
                    </div>
                    <div>
                        <el-tooltip>
                            <template #content>{{t('dao.information.threshold.tips')}}</template>
                            <b>
                                {{t('dao.information.threshold.title')}}
                                <el-icon><QuestionFilled /></el-icon>
                            </b>
                        </el-tooltip>
                        <span>{{proposal.vote_threshold.amount}}</span>
                    </div>
                </div>
            </el-card>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, computed, defineProps, PropType} from 'vue';
    import {ElCard, ElTooltip, ElIcon} from 'element-plus/es';
    import {QuestionFilled, UserFilled, Comment} from '@element-plus/icons-vue';
    import {t} from '@/locale';
    import {ApiDaoProposal, ApiUserInfo} from "@/api/types";
    import Username from '@/components/common/Username.vue';
    import {formatDateToMinutes} from "@/utils/dates";
    import {getTargetUser} from "@/api/user";

    const targetUser = ref<ApiUserInfo>();
    const props = defineProps({
        proposal: {
            type: Object as PropType<ApiDaoProposal>,
            required: true,
        },
    });

    const init = () => {
        getTargetUser(props.proposal.payload.execute_args.AddGovernanceMember.id.toString()).then(res => {
            if (res.Ok) {
                targetUser.value = res.Ok
                // console.log("target",targetUser.value)
            }
        })
    }

    onMounted(() => {
        init()
    });

</script>
<style lang="scss">
    .dao-detail-information-container {
        .dao-form{
            .user{
                float: right;
                display: flex;
                align-items: center;
            }
            b{
                color: #8590a6;
            }
            span{
                float: right;
            }
        }
    }
</style>
