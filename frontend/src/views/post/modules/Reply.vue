<template>
    <div class="post-detail-reply-container" v-infinite-scroll="onScroll" :infinite-scroll-distance="200">
        <div class="container">
            <el-row>
                <el-col :sm={span:16,offset:4} :xs="24">
                    <el-card>
                        <div class="head">
                            <b v-if="list.length === 0 || list.length === 1">
                                {{list.length+ " " + t('post.size') + t('post.answer')}}
                            </b>
                            <b v-else>{{list.length+ " " + t('post.size') + t('post.answers')}}</b>
                        </div>
                        <div class="reply" v-for="(item,index) in showList">
                            <div v-if="props.answerId === Number(item.id)" style="margin-bottom: 5px">
                                <el-tag type="success">
                                    <el-icon><Flag /></el-icon>
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
                                                      ? item.authorData.name: ''"/>
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
                                <div class="flex">
                                    <span v-if="item.comments.length===0 || item.comments.length===1"
                                          @click="openReplyReply(index)">
                                        {{item.comments.length+ " " + t('post.item') + t('post.comment')}}
                                    </span>
                                    <span v-else @click="openReplyReply(index)">
                                        {{item.comments.length+ " " + t('post.item') + t('post.comments')}}
                                    </span>
                                    <span @click="share(item.id)">{{t('common.share')}}</span>
                                    <el-popconfirm v-if="isOwner && props.answerId===undefined"
                                                   :title="t('post.adopt.confirm')"
                                                   :confirmButtonText="t('common.confirm')"
                                                   :cancelButtonText="t('common.cancel')"
                                                   @confirm="submitAnswer(Number(item.post_id),Number(item.id))"
                                    >
                                        <template #reference>
                                            <div class="owner-div flex-y-center">
                                                <el-icon>
                                                    <Medal/>
                                                </el-icon>
                                                <span>{{t('post.adopt.text')}}</span>
                                            </div>
                                        </template>
                                    </el-popconfirm>
                                </div>
                                <div>
                                    <span v-if="!foldIndex[index]" @click="fold(index)">{{t('common.expand')}}</span>
                                    <span v-else @click="fold(index)">{{t('common.fold')}}</span>
                                </div>
                            </div>
                        </div>
                        <div class="reply" v-if="list.length===0">
                            {{t('post.noAnswer')}}
                        </div>
                    </el-card>
                </el-col>
            </el-row>
        </div>
    </div>
    <ReplyReply v-if="showReplyReply" v-model:visible="showReplyReply" :comments="comments" :replyId="commentId"
                :postId="props.postId"
                @refreshCallback="init()"/>
</template>
<script lang="ts" setup>
    import {ref, onMounted, defineProps, defineExpose} from 'vue';
    import {ElRow, ElCol, ElButton, ElCard, ElIcon, ElPopconfirm, ElTag} from 'element-plus/es';
    import {Medal, Flag} from '@element-plus/icons-vue';
    import Avatar from '@/components/common/Avatar.vue';
    import Username from '@/components/common/Username.vue';
    import ReplyReply from './ReplyReply.vue';
    import {ApiPostComments} from "@/api/types";
    import {getTargetUser} from "@/api/user";
    import {getPostComments, submitPostAnswer} from "@/api/post";
    import {t} from '@/locale';
    import {toClipboard} from "@soerenmartius/vue3-clipboard";
    import {showMessageSuccess} from "@/utils/message";
    import {getTimeF} from "@/utils/dates";

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
        }
    });
    const list = ref<ApiPostComments[]>([]);
    const showList = ref<ApiPostComments[]>([]);
    const answer = ref<ApiPostComments>();
    const showReplyReply = ref(false);
    const pageLoading = ref(false);
    const foldIndex = ref([false]);
    const pageSize = ref(5);
    const pageNum = ref(0);
    const totalCount = ref(0);
    const replyIndex = ref(-1);
    const commentId = ref(0);
    const comments = ref<any[]>([]);

    const onScroll = () => {
        //初始化时会运行一次此方法
        //不能加载分页的时候停止请求博客列表，免得陷入死循环
        console.log("onScroll", pageNum.value)
        if (totalCount.value !== 0 && showList.value.length !== totalCount.value) {
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

    const submitAnswer = (postId: number, commentId: number) => {
        submitPostAnswer(postId, commentId).then(res => {
            console.log("res", res)
            if (res.Ok) {
                showMessageSuccess(t('message.post.adopt'))
                //采纳完成后，重新加载页面
                init();
                pageNum.value=0;
                paging();
            } else {
                console.error("submitPostAnswerFailed", res);
            }
        })
    }
    //将指定的回答id置顶
    const onTop = (commentId:number) => {
        const index = list.value.findIndex(item => Number(item.id) === commentId)
        //将查到的答案移到数组第一位，得到置顶的效果。
        list.value.unshift(list.value.splice(index,1)[0])
    }

    const paging = () => {
        if (totalCount.value > 0) {
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
                //由于按时间顺排序问题，需要倒序
                list.value = res.Ok.reverse();
                totalCount.value = list.value.length;
                if (props.answerId) {
                    console.log("props.answerId",props.answerId)
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
        console.log("props",props.answerId)
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
