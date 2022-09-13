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
        <span v-if="voteOption">{{t('dao.vote.yes',{ id: proposalId })}}</span>
        <span v-else-if="!voteOption">{{t('dao.vote.no',{ id: proposalId })}}</span>
        <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{t('common.cancel')}}</el-button>
        <el-button type="primary" @click="vote()" :loading="loading">{{t('common.confirm')}}</el-button>
      </span>
        </template>
    </el-dialog>
</template>
<script lang="ts" setup>
    import {ref} from 'vue';
    import {ElCard, ElButton, ElDialog} from 'element-plus/es';
    import {t} from '@/locale';
    import {useRoute} from "vue-router";
    import {voteProposal} from "@/api/dao";
    import {showMessageError, showMessageSuccess} from "@/utils/message";

    const route = useRoute();

    const proposalId = Number(route.params.id);
    const dialogVisible = ref(false);
    const voteOption = ref(false);
    const loading = ref(false);

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
        voteProposal(proposalId, vote).then(res => {
            console.log("res",res)
            if (res.Ok) {
                showMessageSuccess(t('message.delete.success'))
            } else {
                showMessageError(JSON.stringify(res.Err))
            }
        }).finally(() => {
            loading.value = false
        })
    }

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
