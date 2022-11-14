<template>
    <div class="right-menu-container">
        <el-card shadow="never">
            <el-button type="primary" class="create-button" @click="onClick()"
                       v-if="buttonType==='post'">
                {{t('post.help.create')}}
            </el-button>
            <el-button type="primary" class="create-button" @click="onClick()"
                       v-else-if="buttonType==='dao'"
                       :disabled="isDisabled">
                {{t('dao.create')}}
            </el-button>
            <div class="beta">
                <b class="flex-y-center">
                    <el-icon>
                        <Opportunity/>
                    </el-icon>
                    Beta Warning!
                </b>
                <span class="text">iKnows is in the Beta phase, there may be issues.</span>
            </div>
            <div class="post-top-like" v-if="!error">
                <el-divider content-position="left">
                    <i class="iconfont icon-fire-fill"></i>
                    {{t('common.hot')}}
                </el-divider>
                <el-skeleton :rows="3" :loading="loading" animated>
                    <template #default>
                        <span v-for="item in list" :key="Number(item.id)" @click="goDetail(Number(item.id))"
                              class="post-title">
                            {{item.title}}
                        </span>
                    </template>
                </el-skeleton>
            </div>
            <el-divider/>
            <div class="public-item">
                <a href="https://dfinity.org/">
                    More about the Internet Computer
                </a>
            </div>
        </el-card>
        <Footer/>
    </div>
</template>

<script lang="ts" setup>
    import {defineProps, computed, onMounted, ref} from 'vue';
    import {ElButton, ElCard, ElIcon, ElDivider,ElSkeleton} from 'element-plus/es';
    import {Opportunity} from '@element-plus/icons-vue';
    import Footer from '@/components/footer/Footer.vue';
    import {t} from '@/locale';
    import {useRouter} from 'vue-router';
    import {useStore} from "vuex";
    import {showAdmin} from "@/common/auth";
    import {getTopLikePosts} from "@/api/post";
    import {ApiPost} from "@/api/types";

    const store = useStore();
    const router = useRouter();
    const isDisabled = ref(true);
    const loading = ref(true);
    const error = ref(false);
    const props = defineProps({
        buttonType: {
            type: String,
            required: true,
        },
    });
    const list = ref<ApiPost[]>([]);
    const currentUserPrincipal = computed<string>(() => {
        return store.state.user.principal
    });

    const getTopLikePost = () => {
        getTopLikePosts().then((res) => {
            // console.log("getTopLikePosts", res)
            if (res.Ok) {
                list.value = res.Ok;
                loading.value = false;
            } else {
                error.value = true;
            }
        })
    }

    const init = async () => {
        if (props.buttonType === 'dao') {
            //isDisabled为false时按钮启用，true时按钮禁用
            isDisabled.value = !await showAdmin();
        } else {
            //未登录禁用发贴
            if (!currentUserPrincipal.value) {
                isDisabled.value = true;
            }
            isDisabled.value = false;
        }
    }

    const goDetail = (postId: number) => {
        router.push('/post/detail/' + postId);
    }


    onMounted(() => {
        getTopLikePost();
        init()
    });

    const onClick = () => {
        if (props.buttonType === 'post') {
            router.push('/post/submit')
        } else if (props.buttonType === 'dao') {
            router.push('/dao/submit')
        }
    };

</script>

<style lang="scss">
    .right-menu-container {
        .beta {
            margin-top: 24px;
            padding: 8px 16px;
            color: #e6a23c;
            background-color: #faecd8;
            border-color: #faecd8;
            border-radius: 10px;
            .text {
                font-size: 14px;
            }
        }
        .post-top-like {
            .post-title {
                display: block;
                font-size: 15px;
                cursor: pointer;
                overflow: hidden;
                text-overflow: ellipsis;
                display: -webkit-box;
                -webkit-line-clamp: 2;
                -webkit-box-orient: vertical;
                &:hover {
                    text-decoration: underline;
                }
            }
        }
        .public-item {
            font-size: 14px;
            a {
                color: black;
                text-decoration: none;
                &:hover {
                    opacity: 0.8;
                    text-decoration: underline;
                }
            }
        }
        .create-button {
            width: 100%;
            min-height: 40px;
        }
    }
</style>
