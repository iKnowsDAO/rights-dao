<template>
    <div class="post-detail-head-container">
        <div class="container">
            <el-row>
                <el-col :sm={span:16,offset:4} :xs="24">
                    <div class="post-title">
                        <el-row justify="space-between">
                            <el-col :span="20" class="card-info">
                                <Avatar :username="author && author.name ? author?.name : ''"
                                        :principal-id=post.author.toString()
                                        :avatar-id="Number(author?.avatar_id)"
                                        :clickable="false"
                                        :size="60"/>
                                <div class="text">
                                    <div class="title">
                                        <span>{{post.title}}</span>
                                        <span class="post-status enable" v-if="post.status.Enable!==undefined">{{t('common.status.enable')}}</span>
                                        <span class="post-status completed"
                                              v-else-if="post.status.Completed!==undefined">{{t('common.status.completed')}}</span>
                                        <span class="post-status closed" v-else-if="post.status.Closed!==undefined">{{t('common.status.closed')}}</span>
                                    </div>
                                    <div class="info">
                                        <Username :principalId="post.author.toString()"
                                                  :username="author!==undefined && author.name!==''
                                                      ? author.name: ''"
                                                :clickable="true"/>
                                        <span>|</span>
                                        <span class="createTime">{{getTimeF(Number(post.created_at))}}</span>
                                    </div>
                                    <div class="need-type" v-if="post.participants.length>0">
                                        {{t('post.help.participants.label')}}
                                        <el-tag v-for="(item,index) in post.participants">{{item}}</el-tag>
                                    </div>
                                </div>
                            </el-col>
                            <el-col :span="4" class="flex-right">
                                <el-button type="primary" v-if="post.category.Tech!==undefined">
                                    {{t('post.help.category.tech')}}
                                </el-button>
                                <el-button type="primary" v-else-if="post.category.Law!==undefined">
                                    {{t('post.help.category.law')}}
                                </el-button>
                                <el-button type="primary" v-else>{{t('post.help.category.other')}}</el-button>
                            </el-col>
                        </el-row>
                        <div class="content ql-snow">
                            <div v-if="post.content.format==='html'"
                                 class="ql-editor"
                                 :class="{hidden:isFold}"
                                 ref="htmlInformation"
                                 v-html="post.content.content"
                            >
                            </div>
                            <div v-else>
                                {{post.content.content}}
                            </div>
                        </div>
                        <div class="adopted" v-if="post.answer.length>0">
                            <el-icon>
                                <Flag/>
                            </el-icon>
                            {{t('post.adopt.already')}}
                        </div>
                        <div class="footer">
                            <div>
                                <el-button type="primary" style="margin-right: 5px" @click="writeAnswer">
                                    {{t('post.writeAnswer')}}
                                </el-button>
                                <!--<el-button type="primary" style="margin-right: 5px">发起提案</el-button>-->
                                <!--<span style="margin: 5px;">{{post.comments.length}} 条回复</span>-->
                                <span v-if="isFold" @click="isFold = !isFold" class="fold">{{t('common.expand')}}</span>
                                <span v-else @click="isFold = !isFold" class="fold">{{t('common.fold')}}</span>
                            </div>
                            <div v-if="isOwner">
                                <span class="fold" @click="dialogVisible=true">{{t('common.delete.title')}}</span>
                            </div>
                        </div>
                    </div>
                </el-col>
            </el-row>
        </div>
    </div>
    <el-dialog
        v-model="dialogVisible"
        :title="t('common.delete.title')"
        width="30%"
    >
        <span>{{t('common.delete.text')}}</span>
        <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{t('common.cancel')}}</el-button>
        <el-button type="primary" @click="deleteThisPost()" :loading="loading">{{t('common.confirm')}}</el-button>
      </span>
        </template>
    </el-dialog>
</template>
<script lang="ts" setup>
    import {ref, onMounted, defineProps, PropType, defineEmits} from 'vue';
    import {ElRow, ElCol, ElButton, ElCard, ElTag, ElIcon, ElDialog} from 'element-plus/es';
    import {Flag} from '@element-plus/icons-vue';
    import Avatar from '@/components/common/Avatar.vue';
    import Username from '@/components/common/Username.vue';
    import {ApiPost, ApiUserInfo} from "@/api/types";
    import {getTargetUser} from "@/api/user";
    import {getTimeF} from "@/utils/dates";
    import {t} from '@/locale';
    import {deletePost} from "@/api/post";
    import {showMessageError, showMessageSuccess} from "@/utils/message";
    import {goHome} from "@/router/routers";
    import {useRouter} from "vue-router";

    const router = useRouter();
    const author = ref<ApiUserInfo>();

    const isFold = ref(true);
    const dialogVisible = ref(false);
    const loading = ref(false);

    const props = defineProps({
        post: {
            type: Object as PropType<ApiPost>,
            required: true,
        },
        isOwner: {
            type: Boolean,
            required: true,
        }
    });

    onMounted(() => {
        console.log("a",author.value)
        init();
    });

    const fold = () => {
        isFold.value = !isFold.value;
    }

    const deleteThisPost = () => {
        loading.value = true;
        deletePost(Number(props.post.id)).then(res => {
            if (res.Ok) {
                showMessageSuccess(t('message.delete.success'))
                goHome(router)
            } else {
                showMessageError(res.Err + "")
            }
        }).finally(() => {
            loading.value = false
        })
    }

    const init = () => {
        getTargetUser(props.post.author.toString()).then(res => {
            if (res.Ok) {
                author.value = res.Ok
            }
        })
    }

    const emit = defineEmits(['showWrite'])
    const writeAnswer = () => {
        emit('showWrite');
    }

</script>
<style lang="scss">
    /* 当页面宽度小于426px*/
    @media screen and (max-width:426px) {
        .post-detail-head-container{
            padding: 0 10px;
        }
    }
    .post-detail-head-container {
        background-color: white;
        -webkit-box-shadow: 0 1px 3px rgb(18 18 18 / 10%);
        box-shadow: 0 1px 3px rgb(18 18 18 / 10%);
        span + span {
            margin-left: 10px;
        }
        .fold {
            color: rgb(133, 144, 166);
            margin-left: 10px;
            &:hover {
                cursor: pointer;
            }
        }
        .adopted {
            font-size: 12px;
            color: #9eadb6;
            .el-icon {
                color: rgb(103, 194, 58);
                font-size: 14px;
            }
        }
        .post-title {
            margin-top: 40px;
            .card-info {
                text-align: left;
                display: inherit;
                .text {
                    margin-left: 20px;
                    .info {
                        .createTime {
                            color: rgb(133, 144, 166);
                            font-size: 14px;
                        }
                    }
                }
                .title {
                    font-size: 20px;
                    font-weight: bold;
                }
                .need-type {
                    .el-tag {
                        /*margin-right: 5px;*/
                    }
                }
            }
            .flex-right {
                display: flex;
                justify-content: end;
                align-items: center;
            }
            .footer {
                margin-top: 15px;
                margin-bottom: 15px;
                display: flex;
                justify-content: space-between;
            }
        }
    }
</style>
