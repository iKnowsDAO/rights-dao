<template>
    <div class="post-detail-container">
        <Navigator/>
        <Head :post="post" :isOwner="isOwner" :currentUserPrincipal="currentUserPrincipal" @showWrite="showWriteReply()" v-if="post!==undefined"/>
        <WriteReply @foldWrite="foldWrite(false)" @replySuccess="replyInit" v-show="showWrite"/>
        <div v-if="post!==undefined" style="min-height: 70vh">
            <TimeLine :postId="postId" @changeStatusSuccess="init" :isOwner="isOwner"/>
            <Reply :postId="postId" :answerId="post.answer.length>0 ? Number(post.answer[0]) : undefined" ref="reply"
                   :isOwner="isOwner"
                   :currentUserPrincipal="currentUserPrincipal"/>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, computed} from 'vue';
    import Navigator from '@/components/navigator/Navigator.vue';
    import Head from './modules/Head.vue';
    import TimeLine from './modules/TimeLine.vue';
    import WriteReply from './modules/WriteReply.vue';
    import Reply from './modules/Reply.vue';
    import {ElLoading} from 'element-plus/es';
    import {useRoute, useRouter} from 'vue-router';
    import {getPostInfo} from "@/api/post";
    import {ApiPost} from "@/api/types";
    import {goBack} from "@/router/routers";
    import {showMessageError} from "@/utils/message";
    import {t} from "@/locale";
    import { useUserStore } from "@/stores/user";

    const route = useRoute();
    const router = useRouter();
    const userStore = useUserStore();

    const postId = Number(route.params.id);
    const currentUserPrincipal = computed<string>(() => userStore.principal);
    // 是否是本人 或者是管理员。关联编辑，删除按钮的显示与否
    // 本地环境下，authorId和currentId会有冲突。
    const isOwner = computed(() => {
        if (post.value) {
            return currentUserPrincipal.value === post.value.author.toString()
        }
        return false;
    });
    const post = ref<ApiPost>();
    const reply = ref();
    const loading = ref(false);
    const showWrite = ref(false);

    onMounted(() => {
        init();
    });

    const replyInit = () => {
        reply.value.init();
    };

    const init = () => {
        const fullLoading = ElLoading.service({
            lock: true
        });
        loading.value = true;
        getPostInfo(postId).then(res => {
            console.log("getPostInfo", res);
            if (res.Ok) {
                post.value = res.Ok;
                console.log("detail", isOwner.value)
            } else if (res.Err && res.Err.PostNotFound !== undefined) {
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

    const showWriteReply = () => {
        showWrite.value = !showWrite.value
    }

    const foldWrite = (show: boolean) => {
        showWrite.value = show
    }

</script>
<style lang="scss">
    .post-detail-container {
        background-color: rgb(246, 246, 246);
    }
</style>
