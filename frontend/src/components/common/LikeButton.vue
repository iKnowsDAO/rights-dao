<template>
    <div class="like-button flex-y-center"
         :class="{ 'like-button-not-clickable': isDisabled }"
         @click.stop="onClick">
        <i v-if="!isLiked" class="iconfont icon-like"></i>
        <i v-else class="iconfont icon-like-filled"></i>
        {{props.likeCount}}
    </div>
</template>

<script lang="ts" setup>
    import {ref, defineProps, onMounted, computed} from 'vue';
    import {cancelLike, isLikedPost, likePost} from "@/api/post";
    import { useUserStore } from "@/store/user";

    const userStore = useUserStore();
    const props = defineProps({
        // 问题ID
        postId: {
            type: Number,
            required: true,
        },
        // 评论id
        commentId: {
            type: Number,
        },
        likeCount: {
            type: Number,
            required: true,
            default: 0
        },
    });
    const currentUserPrincipal = computed<string>(() => {
        if (!userStore.principal) {
            isDisabled.value = true;
        }
        return userStore.principal
    });
    const isDisabled = ref(false);
    const isLiked = ref(false);

    const onClick = async () => {
        //根据是否点赞执行不同的方法
        //没登录不允许点赞
        if (isDisabled.value) {
            return
        }
        if (isLiked.value) {
            //取消点赞
            cancelLike(props.postId, true, props?.commentId).then(res => {
                if (res.Ok) {

                } else {
                    //失败回滚
                    isLiked.value = true;
                }
            })
            isLiked.value = !isLiked.value;
            if (props.likeCount > 0) {
                //@ts-ignore
                props.likeCount--;
            }
        } else {
            //没有点赞，就点赞
            likePost(props.postId, true, props?.commentId).then(res => {
                if (res.Ok) {

                } else {
                    //失败回滚
                    isLiked.value = false;
                }
            })
            isLiked.value = !isLiked.value;
            //@ts-ignore
            props.likeCount++;
        }
        clickLocked();
    }

    const isUserLiked = () => {
        if (currentUserPrincipal.value && isDisabled.value) {
            return
        }
        isLikedPost(props.postId, true, props?.commentId).then(res => {
            if (isLiked.value) {
                //如果用户已经点赞过，不进行以下操作。
                return
            }
            if (res.Ok) {
                isLiked.value = true;
            } else {
                isLiked.value = false;
            }
        })
    }

    const clickLocked = () => {
        if (isDisabled.value) {
            //如果已经是禁用状态就不锁定
            return;
        }
        isDisabled.value = true;
        setTimeout(() => {
            //计时器，点赞后1.5s内不能再点赞
            isDisabled.value = false;
        }, 1500);
    }

    onMounted(() => {
        isUserLiked();
    });

</script>

<style lang="scss" scoped>
    .like-button {
        .iconfont {
            cursor: pointer;
            font-size: 20px;
            color: #8590a6;
        }
    }
    .like-button-not-clickable {
        cursor: not-allowed !important;
        i {
            cursor: not-allowed !important;
        }
    }
</style>
