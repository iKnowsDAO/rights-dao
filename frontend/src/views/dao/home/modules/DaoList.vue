<template>
    <div class="dao-list-container">
        <div class="container">
            <div class="post-list">
                <el-row :gutter="20">
                    <el-col :sm="19" :xs="24">
                        <div style="display: flex">
                            <el-input
                                v-model="search"
                                class="search"
                                :placeholder="t('post.help.search')"
                                :prefix-icon="Search"
                                clearable
                            />
                            <el-button type="primary" @click="searchPage()" style="margin-left: 10px">
                                {{t('common.search')}}
                            </el-button>
                        </div>
                        <el-card class="post-card" v-for="(item,inex) in showList"
                                 @click="onClick(Number(item.id))">
                            <el-row justify="space-between">
                                <el-col :span="20" class="card-info">
                                    <Avatar :username="item.authorData && item.authorData.name!=='' ?
                                                item.authorData.name : item.proposer.toString()"
                                            :principalId=item.proposer.toString()
                                            :clickable="false"
                                            :size="60"/>
                                    <div class="text">
                                        <div class="title">
                                            <span @click="onClick(Number(item.id))">{{item.title}}</span>
                                            <DaoState :state="item.state"/>
                                        </div>
                                        <div class="info">
                                            <Username :principalId="item.proposer.toString()"
                                                      :username="item.authorData && item.authorData.name!==''
                                                      ? item.authorData.name: ''"/>
                                            <span>|</span>
                                            <span class="createTime">{{getTimeF(Number(item.created_at))}}</span>
                                        </div>
                                    </div>
                                </el-col>
                                <el-col :span="4" class="flex-right">
                                   <span>category</span>
                                </el-col>
                            </el-row>
                            <div @click="onClick(Number(item.id))" class="content">
                                {{item.content.content}}
                            </div>
                            <div class="footer">
                                <!--<div class="reply">-->
                                    <!--{{t('post.reply')+" "+item.comments.length}}-->
                                <!--</div>-->
                            </div>
                        </el-card>
                        <el-row :class="{ empty: list.length === 0 }" justify="center" class="loading-tip">
                            <div class="note" v-if="pageLoading">
                                {{ $t('common.loading') }}
                            </div>
                            <div class="note" v-else-if="totalCount === 0 || totalCount === list.length">
                                {{ $t('common.noMore') }}
                            </div>
                        </el-row>
                    </el-col>
                    <el-col :sm="5" :xs="24">
                        <RightMenu buttonType="dao"/>
                    </el-col>
                </el-row>
            </div>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, computed} from 'vue';
    import {t} from '@/locale';
    import {
        ElRow,
        ElCol,
        ElInput,
        ElButton,
        ElCard,
        ElTag,
        ElIcon,
        ElDivider,
        ElMenu,
        ElMenuItem
    } from 'element-plus/es';
    import {Search, Opportunity} from '@element-plus/icons-vue'
    import Avatar from '@/components/common/Avatar.vue';
    import Username from '@/components/common/Username.vue';
    import RightMenu from '@/components/menu/RightMenu.vue';
    import DaoState from '@/components/common/dao/DaoState.vue';
    import {useRoute, useRouter} from 'vue-router';
    import {getTimeF} from "@/utils/dates";
    import {ApiDaoProposal} from "@/api/types";
    import {cleanHtml} from "@/common/utils";
    import {getPageDaoProposal} from "@/api/dao";
    import {getTargetUser} from "@/api/user";

    const router = useRouter();
    const search = ref("");
    const pageSize = ref(5);
    const pageNum = ref(0);
    const totalCount = ref(0);
    const pageLoading = ref(false);
    const list = ref<ApiDaoProposal[]>([]);

    const onClick = (id: number) => {
        router.push('/dao/detail/' + id);
    }

    // 过滤显示的内容
    const showList = computed<Recordable<any>[]>(() => {
        return list.value.map((item) => {
            return {
                ...item,
                title: item.payload.execute_args.AddGovernanceMember.title,
                content: {
                    //移除html标签
                    content: item.payload.execute_args.AddGovernanceMember.content
                        ? cleanHtml(item.payload.execute_args.AddGovernanceMember.content.content)
                        : item.payload.execute_args.AddGovernanceMember.content,
                    format: "html"
                }
            };
        });
    });

    const onScroll = () => {
        //初始化时会运行一次此方法，所以懒加载分页从1开始
        //不能加载分页的时候停止请求博客列表，免得陷入死循环
        if (totalCount.value !== 0 && list.value.length !== totalCount.value) {
            pageNum.value++;
            init(false)
        }
    };

    const searchPage = () => {
        pageNum.value = 0;
        list.value = [];
    }

    //isClean，是否在收集到返回值后，清空之前的list（用于切换板块）
    const init = (isClean: boolean) => {
        pageLoading.value = true;
        // console.log("dao",pageNum.value, pageSize.value, search.value)
        getPageDaoProposal(pageNum.value, pageSize.value, search.value).then(res => {
            console.log("dao", res)
            if (res.Ok) {
                totalCount.value = Number(res.Ok.total_count);
                const length = list.value.length;
                list.value.push(...res.Ok.data);
                //需要获取user数据的index区间在(length, length + res.Ok.data.length)
                for (let i = length; i < list.value.length; i++) {
                    getTargetUser(list.value[i].proposer.toString()).then(res => {
                        if (res.Ok) {
                            list.value[i] = {
                                ...list.value[i],
                                authorData: res.Ok,
                            }
                        }
                    })
                }
            }
        }).finally(() => {
            pageLoading.value = false;
        })
    }

    onMounted(() => {
        init(false)
    });
</script>
<style lang="scss">
    .dao-list-container {
        padding-top: 24px;
        .empty {
            margin-top: 200px;
        }
        .loading-tip {
            margin-top: 30px;
            margin-bottom: 30px;
        }
        .container {
            .flex-right {
                display: flex;
                justify-content: end;
                align-items: center;
            }
            .post-card {
                text-align: left;
                margin-top: 20px;
                &:hover {
                    cursor: pointer;
                }
                .el-card__body {
                    /*padding: 20px 60px;*/
                }
                .card-info {
                    display: inherit;
                    .text {
                        margin-left: 20px;
                    }
                    .info {
                        span + span {
                            margin-left: 10px;
                        }
                        .createTime {
                            color: rgb(133, 144, 166);
                            font-size: 16px;
                        }
                    }
                    .title {
                        font-size: 20px;
                        font-weight: bold;
                        span:first-child:hover {
                            cursor: pointer;
                        }
                        span + span {
                            margin-left: 10px;
                        }
                    }
                }
                .content {
                    margin-top: 10px;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    display: -webkit-box;
                    -webkit-line-clamp: 6;
                    -webkit-box-orient: vertical;
                    &:hover {
                        cursor: pointer;
                    }
                }
                .footer {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-top: 25px;
                    .reply {
                        color: rgb(133, 144, 166);
                    }
                }
            }
        }
    }
</style>
