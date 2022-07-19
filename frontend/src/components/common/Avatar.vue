<template>
    <div class="avatar-container">
        <el-avatar
            :src="imageSrc"
            :size="Number(size)"
            :style="{ fontSize: Number(size) / 2 + 'px', backgroundColor }"
            :class="{ 'avatar-clickable': clickable }"
            @click="onClick"
            :shape="shape"
        >
            {{ showAvatar }}
        </el-avatar>
    </div>
</template>

<script lang="ts" setup>
import { PropType, defineProps, ref, computed, watch, onMounted } from 'vue';
import { extractColorByName, showAvatarName } from '@/utils/avatars';
import { ElAvatar } from 'element-plus/es';
import { openTab } from '@/router/routers';
import { getUrlByPhotoServiceId } from '@/utils/images';

const props = defineProps({
    // 必要的内容，显示哪些
    principalId: {
        type: String,
        required: true,
    },
    username: {
        type: String,
        required: true,
    },
    avatarId: Number,
    avatarUri: String,
    // 提供调节大小和样式
    size: {
        type: Number,
        default: 40,
    },
    shape: {
        type: String as PropType<'circle' | 'square'>,
        default: 'circle',
    },
    // 是否可以点击（hover时鼠标变手）
    clickable: {
        type: Boolean,
        default: false,
    },
    clickCallback: Function,
});

const imageSrc = ref('');

const showAvatar = computed<string>(() => {
    const m = showAvatarName(props.username, props.principalId);
    return m ? m : 'A';
});
// 根据名字，定义头像颜色
const backgroundColor = computed<string>(() => {
    return extractColorByName(props.username);
});

watch(
    () => props.avatarId,
    () => {
        if (!props.avatarId) imageSrc.value = '';
        getAvatarSrc();
    },
);

watch(
    () => props.avatarUri,
    () => {
        if (!props.avatarUri) imageSrc.value = '';
        getAvatarSrc();
    },
);

onMounted(() => {
    getAvatarSrc();
});

const getAvatarSrc = () => {
    // 根据id读取头像图片
    if (props.avatarUri) {
        imageSrc.value = props.avatarUri;
    } else if (props.avatarId) {
        imageSrc.value = getUrlByPhotoServiceId(props.avatarId);
    } else {
        imageSrc.value = '';
    }
};

const onClick = () => {
    if (!props.clickable) return;
    // 应该有点击响应
    if (props.clickCallback) {
        props.clickCallback.call(null);
    } else {
        openTab(location.origin + '/person/profile/' + props.principalId);
    }
};
</script>

<style lang="scss">
.avatar-container {
    display: flex;
    justify-content: center;
    align-items: center;
    .avatar-clickable {
        cursor: pointer;
    }
}
</style>
