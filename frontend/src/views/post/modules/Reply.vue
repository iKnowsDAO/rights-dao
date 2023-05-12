<template>
    <div class="post-detail-reply-container" v-infinite-scroll="onScroll" :infinite-scroll-distance="200">
        <div class="container">
            <el-row>
                <el-col :sm={span:16,offset:4} :xs="24">
                    <el-card>
                        <div class="head">
                            <b v-if="total === 0 || total === 1">
                                {{total+ " " + t('post.size') + t('post.answer')}}
                            </b>
                            <b v-else>{{total+ " " + t('post.size') + t('post.answers')}}</b>
                        </div>
                        <div class="reply" v-for="(item,index) in showList">
                            <div v-if="props.answerId === Number(item.id)" style="margin-bottom: 5px">
                                <el-tag type="success">
                                    <el-icon>
                                        <Flag/>
                                    </el-icon>
                                    {{t('post.adopt.down')}}
                                </el-tag>
                            </div>
                            <div class="author">
                                <div class="flex">
                                    <Avatar :username="item.authorData && item.authorData.name!=='' ?
                                            item.authorData.name : item.author.toString()"
                                            :principalId=item.author.toString()
                                            :size="38"/>
                                    <div class="authorName">
                                        <b>
                                            <Username :principalId="item.author.toString()"
                                                      :username="item.authorData && item.authorData.name!==''
                                                      ? item.authorData.name: ''"
                                                      :sbtLevel="item.authorData && item.authorData.claimed_sbt[0] ?
                                                      item.authorData.claimed_sbt[0].medal.level : 0"
                                                      :clickable="true"/>
                                        </b>
                                        <div class="sign" v-if="item.authorData && item.authorData.biography!==''">
                                            {{item.authorData.biography}}
                                        </div>
                                    </div>
                                </div>
                                <span class="create-time">{{getTimeF(Number(item.created_at))}}</span>
                            </div>
                            <div class="content ql-snow">
                                <div v-if="item.content.format==='html'"
                                     class="ql-editor"
                                     :class="{hidden:!foldIndex[index]}"
                                     ref="htmlInformation"
                                     v-html="item.content.content"
                                >
                                </div>
                                <div v-else>
                                    {{item.content.content}}
                                </div>
                            </div>
                            <div class="footer">
                                <div class="left flex-y-center">
                                    <span v-if="item.comments.length===0 || item.comments.length===1"
                                          @click="openReplyReply(index)">
                                        {{item.comments.length+ " " + t('post.item') + t('post.comment')}}
                                    </span>
                                    <span v-else @click="openReplyReply(index)">
                                        {{item.comments.length+ " " + t('post.item') + t('post.comments')}}
                                    </span>
                                    <span @click="share(item.id)">{{t('common.share')}}</span>
                                    <!--必须是发贴人且问题没有采纳答案，并且不能自己采纳自己-->
                                    <el-popconfirm v-if="props.isOwner && props.answerId===undefined && props.currentUserPrincipal!==item.author.toString()"
                                                   :title="t('post.adopt.confirm')"
                                                   :confirmButtonText="t('common.confirm')"
                                                   :cancelButtonText="t('common.cancel')"
                                                   @confirm="submitAnswer(Number(item.post_id),Number(item.id))"
                                    >
                                        <template #reference>
                                            <div>
                                                <el-tooltip :content="t('wallet.bounty.noWallet')"
                                                            :disabled="!(item.authorData?.wallet_principal.length === 0)">
                                                    <div class="owner-div flex-y-center">
                                                        <el-icon>
                                                            <Medal/>
                                                        </el-icon>
                                                        <span>{{t('post.adopt.text')}}</span>
                                                    </div>
                                                </el-tooltip>
                                            </div>
                                        </template>
                                    </el-popconfirm>
                                    <LikeButton :postId="Number(props.postId)" :commentId="Number(item.id)"
                                                :likeCount="Number(item.likes_count)"
                                                style="margin-left:10px"/>
                                    <el-tooltip :content="t('wallet.reward.onlyWallet')">
                                        <div v-if="item.authorData && item.authorData.wallet_principal.length>0"
                                             class="reward-button"
                                             @click="showReward(item.authorData?.wallet_principal)">
                                            <el-icon>
                                                <Coin/>
                                            </el-icon>
                                            {{t('wallet.reward.title')}}
                                        </div>
                                    </el-tooltip>
                                    <DeleteButton v-if="props.currentUserPrincipal===item.author.toString()"
                                                  :id="Number(item.id)"
                                                  :deleteFunction="deleteAnswer"
                                                  :loading="deleteLoading"/>
                                </div>
                                <div class="right">
                                    <span v-if="!foldIndex[index]" @click="fold(index)">{{t('common.expand')}}</span>
                                    <span v-else @click="fold(index)">{{t('common.fold')}}</span>
                                </div>
                            </div>
                        </div>
                        <div class="reply" v-if="total===0">
                            {{t('post.noAnswer')}}
                        </div>
                    </el-card>
                </el-col>
            </el-row>
        </div>
    </div>
    <ReplyReply v-if="showReplyReply" v-model:visible="showReplyReply" :comments="comments" :replyId="commentId"
                :postId="props.postId" :isOwner="props.isOwner" :currentUserPrincipal="props.currentUserPrincipal"
                @refreshCallback="init()"/>
    <TransferDialog
        v-if="dialogVisible"
        v-model:visible="dialogVisible" :title="t('wallet.reward.title')"
        :content="t('wallet.reward.content')"
        :onlyTransfer="true"
        :targetPrincipal="targetPrincipal"/>
</template>
<script lang="ts" setup>
    import { ref, onMounted, defineProps, defineExpose } from 'vue';
    import { ElRow, ElCol, ElButton, ElCard, ElIcon, ElPopconfirm, ElTag, ElTooltip } from 'element-plus/es';
    import { Medal, Flag, Coin } from '@element-plus/icons-vue';
    import Avatar from '@/components/common/Avatar.vue';
    import Username from '@/components/common/Username.vue';
    import DeleteButton from '@/components/common/PostDeleteButton.vue';
    import LikeButton from '@/components/common/LikeButton.vue';
    import ReplyReply from './ReplyReply.vue';
    import TransferDialog from '@/components/wallet/TransferDialog.vue';
    import { ApiPostComments } from "@/api/types";
    import { getTargetUser } from "@/api/user";
    import { deletePostAnswer, getPostComments, submitPostAnswer } from "@/api/post";
    import { t } from '@/locale';
    import { toClipboard } from "@soerenmartius/vue3-clipboard";
    import { showMessageSuccess, showResultError } from "@/utils/message";
    import { getTimeF } from "@/utils/dates";

    const props = defineProps({
        postId: {
            type: Number,
            required: true,
        },
        answerId: {
            type: Number,
        },
        isOwner: {
            type: Boolean,
            required: true,
        },
        currentUserPrincipal: {
            type: String,
            required: true,
        }
    });
    const list = ref<ApiPostComments[]>([]); //初始数据，可能会有很多数据量，所以需要分页成showList
    const showList = ref<ApiPostComments[]>([]); //实际展示数据
    const answer = ref<ApiPostComments>();
    const dialogVisible = ref(false);
    const showReplyReply = ref(false);
    const pageLoading = ref(false);
    const deleteLoading = ref(false);
    const foldIndex = ref([false]);
    const pageSize = ref(5);
    const pageNum = ref(0);
    const total = ref(0);
    const replyIndex = ref(-1);
    const commentId = ref(0);
    const comments = ref<any[]>([]);
    const targetPrincipal = ref("");

    const onScroll = () => {
        //初始化时会运行一次此方法
        //不能加载分页的时候停止请求博客列表，免得陷入死循环
        // console.log("onScroll", pageNum.value)
        if (total.value !== 0 && showList.value.length !== total.value) {
            pageNum.value++;
            paging()
        }
    };

    const fold = (index: number) => {
        foldIndex.value[index] = !foldIndex.value[index];
    }

    const openReplyReply = (index: number) => {
        //打开评论列表
        replyIndex.value = index;
        comments.value = list.value[index].comments;
        showReplyReply.value = true;
        commentId.value = Number(list.value[index].id);
    }

    const share = async (id: bigint) => {
        try {
            await toClipboard(window.location.href)
            showMessageSuccess(t('message.share.success'))
        } catch (e) {
            console.error(e)
        }
    }

    const showReward = (wallet_principal?: string[]) => {
        //有绑定钱包才能打开打赏
        if (wallet_principal && wallet_principal.length > 0) {
            targetPrincipal.value = wallet_principal[0].toString();
            dialogVisible.value = true;
        }
    }

    const deleteAnswer = (answerId: number, callback) => {
        deleteLoading.value = true;
        deletePostAnswer(props.postId, answerId).then(res => {
            console.log("deletePostComment res", res)
            if (res.Ok) {
                showMessageSuccess(t('message.delete.success'))
                //删除成功后将此评论直接移除，并且将总数减1。
                showList.value = showList.value.filter(item => Number(item.id) !== answerId)
                total.value = total.value - 1;
            } else {
                showResultError(res)
            }
            callback(res);
        }).finally(() => {
            deleteLoading.value = false
        })
    }

    const submitAnswer = (postId: number, commentId: number) => {
        submitPostAnswer(postId, commentId).then(res => {
            console.log("res", res)
            if (res.Ok) {
                showMessageSuccess(t('message.post.adopt'))
                //采纳完成后，重新加载页面
                init();
                pageNum.value = 0;
                paging();
            } else {
                console.error("submitPostAnswerFailed", res);
            }
        })
    }
    //将指定的回答id置顶
    const onTop = (commentId: number) => {
        const index = list.value.findIndex(item => Number(item.id) === commentId)
        //将查到的答案移到数组第一位，得到置顶的效果。
        list.value.unshift(list.value.splice(index, 1)[0])
    }

    const paging = () => {
        if (total.value > 0) {
            const length = showList.value.length;
            showList.value.push(...list.value.slice(pageSize.value * (pageNum.value - 1), pageSize.value * pageNum.value));
            console.log("ReplyShowList", showList.value)
            //需要获取user数据的index区间在(length, length + pageSize)
            for (let i = length; i < showList.value.length; i++) {
                getTargetUser(showList.value[i].author.toString()).then(res => {
                    if (res.Ok) {
                        showList.value[i] = {
                            ...showList.value[i],
                            authorData: res.Ok,
                        }
                    }
                })
            }
        }
    }

    const init = async () => {
        await getPostComments(props.postId).then(res => {
            if (res.Ok) {
                console.log("getPostComments", res)
                //由于按时间顺排序问题，需要倒序。
                //倒序后按点赞数大小排序。
                list.value = res.Ok.reverse().sort((a, b) => {
                    return Number(b.likes_count[0]) - Number(a.likes_count[0]);
                });
                total.value = list.value.length;
                if (props.answerId) {
                    console.log("props.answerId", props.answerId)
                    onTop(props.answerId);
                }
            }
        })
        if (list.value.length > 0 && replyIndex.value !== -1 && showReplyReply.value) {
            //如果replyIndex不为0，说明用户目前在看评论区，需要重新加载一下评论区的数据
            openReplyReply(replyIndex.value);
        } else {
            //不在看评论区，得清空分页数据重新来，免得写回答后没有反应。
            showList.value = [];
            pageNum.value = 0;
            onScroll();
        }
    }

    onMounted(() => {
        init();
    });

    defineExpose({
        init
    })

</script>
<style lang="scss">
    .post-detail-reply-container {
        .el-card {
            margin-top: 10px;
            .el-card__body {
                /*padding-left: 30px;*/
                /*padding-right: 30px;*/
            }
            .reply {
                margin-top: 10px;
                padding-top: 10px;
                border-top: 1px solid rgb(246, 246, 246);
                .author {
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    .authorName {
                        margin-left: 10px;
                        display: flex;
                        justify-content: center;
                        flex-direction: column;
                    }
                }
                .owner-div {
                    margin-left: 10px;
                }
                .delete-button {
                    margin-left: 10px;
                }
                .reward-button {
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    margin-left: 10px;
                }
                .footer {
                    display: flex;
                    justify-content: space-between;
                    color: rgb(133, 144, 166);
                    span:hover {
                        cursor: pointer;
                    }
                    span + span {
                        margin-left: 10px;
                    }
                }
            }
        }
    }
</style>
