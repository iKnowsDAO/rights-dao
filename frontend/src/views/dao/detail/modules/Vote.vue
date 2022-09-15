<template>
    <div class="dao-detail-vote-container">
        <div class="container">
            <el-card>
                <template #header>
                    <div class="header">
                        <h4><b>{{t('dao.vote.title')}}</b></h4>
                    </div>
                </template>
                <div class="vote">
                    <el-button type="primary" round @click="openVote(true)">{{t('dao.result.yes')}}</el-button>
                    <el-button type="danger" round @click="openVote(false)">{{t('dao.result.no')}}</el-button>
                </div>
            </el-card>
        </div>
    </div>
    <el-dialog
        v-model="dialogVisible"
        :title="t('dao.vote.dialogTitle')"
        width="30%"
    >
        <span v-if="voteOption">{{t('dao.vote.yes',{ id: proposalId,votePower: votePower })}}</span>
        <span v-else-if="!voteOption">{{t('dao.vote.no',{ id: proposalId,votePower: votePower })}}</span>
        <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{t('common.cancel')}}</el-button>
        <el-button type="primary" @click="vote()" :loading="loading">{{t('common.confirm')}}</el-button>
      </span>
        </template>
    </el-dialog>
</template>
<script lang="ts" setup>
    import {ref, defineProps, onMounted} from 'vue';
    import {ElCard, ElButton, ElDialog} from 'element-plus/es';
    import {t} from '@/locale';
    import {getMemberVote, getMyVotePower, voteProposal} from "@/api/dao";
    import {showMessageSuccess, showResultError} from "@/utils/message";

    const votePower = ref(0);
    const dialogVisible = ref(false);
    const voteOption = ref(false);
    const loading = ref(false);
    const props = defineProps({
        principalId: {
            type: String,
            required: true,
        },
        proposalId: {
            type: Number,
            required: true,
        },
    });

    const openVote = (vote: boolean) => {
        dialogVisible.value = true;
        voteOption.value = vote;
    }

    const vote = () => {
        loading.value = true;
        let vote;
        if (voteOption.value) {
            vote = {Yes: null}
        } else {
            vote = {No: null}
        }
        voteProposal(props.proposalId, vote).then(res => {
            console.log("res",res)
            if (res.Ok) {
                showMessageSuccess(t('message.delete.success'))
            } else {
                showResultError(res)
            }
        }).finally(() => {
            loading.value = false
        })
    }
    const init = () => {
        getMemberVote(props.proposalId, props.principalId).then(res => {
            console.log("getVote", res)
        })
        getMyVotePower().then(res => {
            if (res.Ok) {
                votePower.value = Number(res.Ok.amount)
            }
            console.log("getVotePower", res)
        })
    }

    onMounted(() => {
        init()
    });

</script>
<style lang="scss">
    .dao-detail-vote-container {
        .vote{
            .el-button{
                width: 100%;
                margin-bottom: 8px;
                height: 44px;
                span{
                    font-size: 18px;
                    font-weight: 600;
                }
            }
            .el-button+.el-button{
                margin-left: 0;
            }
        }
    }
</style>
