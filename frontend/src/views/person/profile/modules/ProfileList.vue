<template>
    <div class="person-profile-list-container" v-infinite-scroll="onScroll">
        <div class="container">
            <el-row>
                <el-tabs v-model="option" @tab-click="handleClick">
                    <el-tab-pane :label="t('post.topics')" name="topics"></el-tab-pane>
                    <el-tab-pane :label="t('post.reply')" name="reply"></el-tab-pane>
                </el-tabs>
            </el-row>
            <div class="list" v-if="list.length>0">
                <div v-for="(item,index) in list" class="list-post">
                    <el-card class="user-post-card" shadow="hover" @click="onClick(Number(item.id))">
                        <div class="title">{{item.title}}</div>
                        <div class="content">
                            <span v-if="props.username!=='' && option==='reply'">{{props.username+" ："}}</span>
                            {{item.content.content}}
                        </div>
                        <el-row justify="space-between" class="footer">
                            <div>
                                <span v-if="option==='topics'">{{t('post.reply')+" "+item.length}}</span>
                            </div>
                            <div>
                                <span>{{getTimeF(Number(item.created_at))}}</span>
                            </div>
                        </el-row>
                    </el-card>
                </div>
            </div>
            <el-row :class="{ empty: list.length === 0 }" justify="center" class="loading-tip">
                <div class="note" v-if="loading">
                    {{ $t('common.loading') }}
                </div>
                <div class="note" v-else-if="totalCount === 0 || totalCount === list.length">
                    {{ $t('common.noMore') }}
                </div>
            </el-row>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, defineProps} from 'vue';
    import {t} from '@/locale';
    import {useRoute, useRouter} from 'vue-router';
    import {ElRow, ElCol, ElInput, ElButton, ElCard, ElTag, ElTabs, ElTabPane} from 'element-plus/es';
    import {Search} from '@element-plus/icons-vue'
    import {getTimeF} from "@/utils/dates";
    import {getTargetUserComments, getTargetUserPost, getTargetUserPostComments} from "@/api/user";
    import {ApiProfilePost} from "@/api/types";
    import {cleanHtml} from "@/common/utils";

    const props = defineProps({
        username: {
            type: String,
            required: true,
        },
    });

    const router = useRouter();
    const route = useRoute();
    const pageSize = ref(5);
    const pageNum = ref(0);
    const totalCount = ref(0);
    const query = ref(''); //查询，暂时没用
    const option = ref('topics');
    const loading = ref(false);
    const list = ref<ApiProfilePost[]>([]);

    const targetPrincipal = ref('');

    onMounted(() => {
        targetPrincipal.value = route.params.principal.toString() || '';
        init()
    });

    const init = () => {
        switch (option.value) {
            case 'topics':
                getPostComment();
                break;
            case 'reply':
                getComments();
                break;
        }
    }

    const onClick = (id: number) => {
        router.push('/post/detail/' + id);
    }

    const onScroll = () => {
        //初始化时会运行一次此方法，所以懒加载分页从1开始
        //不能加载分页的时候停止请求博客列表，免得陷入死循环
        console.log("pageNum", pageNum.value)
        if (totalCount.value !== 0 && list.value.length !== totalCount.value) {
            pageNum.value++;
            init()
        }
    }

    const handleClick = () => {
        list.value = [];
        pageNum.value = 0;
        init()
    }

    const getPostComment = () => {
        //获取目标用户发布的主题，现在不包括评论，只包括评论数
        loading.value = true;
        getTargetUserPostComments(pageNum.value, pageSize.value, query.value, targetPrincipal.value).then(res => {
            console.log("userPost", res)
            if (res.Ok) {
                totalCount.value = Number(res.Ok.total_count);
                list.value.push(...res.Ok.data.map(item => {
                    return {
                        id: item.id,
                        created_at: item.created_at,
                        title: item.title,
                        content: {
                            content: cleanHtml(item.content.content),
                            format: "text" as 'text'
                        },
                        length: item.comment_count ? Number(item.comment_count[0]) : 0
                    }
                }))
            }
        }).finally(() => {
            loading.value = false;
        })
    }

    const getComments = () => {
        //获取目标用户发布的评论
        loading.value = true;
        getTargetUserComments(pageNum.value, pageSize.value, query.value, targetPrincipal.value).then(res => {
            console.log("Comments", res)
            if (res.Ok) {
                totalCount.value = Number(res.Ok.total_count);
                list.value.push(...res.Ok.data.map(item => {
                    return {
                        id: item.post_id ? item.post_id : item.id,
                        created_at: item.created_at,
                        title: item.post_title ? item.post_title : "",
                        content: {
                            content: cleanHtml(item.content.content),
                            format: "text" as 'text'
                        },
                        length: 0
                    }
                }))
            }
        }).finally(() => {
            loading.value = false;
        })
    }


</script>
<style lang="scss">
    .person-profile-list-container {
        margin-top: 30px;
        padding: 20px;
        background-color: rgb(246, 246, 246);
        min-height: 70vh;
        @media screen and (max-width:426px) {
            .container{
                padding:unset!important;
            }
        }
        .empty{
            /*margin-top: 10px;*/
        }
        .container {
            background-color: rgb(246, 246, 246);
            padding: 20px 120px;
        }
        .list {
            padding: 30px;
            .user-post-card {
                margin-bottom: 10px;
                text-align: left;
                &:hover {
                    cursor: pointer;
                }
                .el-card__body >div + div {
                    margin-top: 10px;
                }
                .title {
                    font-size: 20px;
                    font-weight: bold;
                }
                .content {
                    overflow: hidden;
                    text-overflow: ellipsis;
                    display: -webkit-box;
                    -webkit-line-clamp: 3;
                    -webkit-box-orient: vertical;
                }
                .footer {
                    color: rgb(133, 144, 166);
                }
            }
        }
    }
</style>
