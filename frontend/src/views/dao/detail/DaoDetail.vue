<template>
    <div class="dao-detail-container">
        <Navigator/>
        <div class="container" v-if="proposal!==undefined">
            <el-row class="main" :gutter="20">
                <el-col class="main-content" :sm="16" :xs="24">
                    <Head :proposal="proposal" v-if="proposal!==undefined"/>
                </el-col>
                <el-col class="sub-content" :sm="8" :xs="24">
                    <Information :proposal="proposal" v-if="proposal!==undefined"/>
                    <Result v-if="proposal!==undefined"
                            :yes="Number(proposal.votes_yes.amount)"
                            :no="Number(proposal.votes_no.amount)"
                            :threshold="Number(proposal.vote_threshold.amount)"/>
                </el-col>
            </el-row>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, computed} from 'vue';
    import Navigator from '@/components/navigator/Navigator.vue';
    import Head from './modules/Head.vue';
    import Information from './modules/Information.vue';
    import Result from './modules/Result.vue';
    import {ElRow, ElCol, ElLoading} from 'element-plus/es';
    import {useRoute, useRouter} from 'vue-router';
    import {ApiDaoProposal} from "@/api/types";
    import {useStore} from "vuex";
    import {goBack} from "@/router/routers";
    import {showMessageError} from "@/utils/message";
    import {t} from "@/locale";
    import {getDaoProposal} from "@/api/dao";

    const route = useRoute();
    const router = useRouter();
    const store = useStore();
    const proposalId = Number(route.params.id);
    const currentUserPrincipal = computed<string>(() => store.state.user.principal);
    // 是否是本人。关联编辑按钮的显示与否
    // 本地环境下，authorId和currentId会有冲突。
    // const isOwner = computed(() => {
    //     if (dao.value) {
    //         return currentUserPrincipal.value === dao.value.author.toString()
    //     }
    //     return false;
    // });
    const proposal = ref<ApiDaoProposal>();
    const reply = ref()
    const loading = ref(false);

    onMounted(() => {
        init();
    });

    const replyInit = () => {
        reply.value.init();
    }

    const init = () => {
        const fullLoading = ElLoading.service({
            lock: true
        });
        loading.value = true;
        getDaoProposal(proposalId).then(res => {
            console.log("getDaoProposal", res)
            if (res.Ok) {
                proposal.value = res.Ok
            } else if (res.Err && res.Err.ProposalNotFound !== undefined) {
                showMessageError(t('message.error.noTarget'));
                setTimeout(() => {
                    //等用户看清了错误提示再弹
                    goBack(router);
                }, 1500);
            }
        }).finally(() => {
            fullLoading.close();
            loading.value = false
        })
    }

</script>
<style lang="scss">
    .dao-detail-container {
        background-color: rgb(246, 246, 246);
        .container{
            margin-top: 10px;
        }
    }
</style>
