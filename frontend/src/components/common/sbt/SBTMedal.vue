<template>
    <img v-if="medalImage" :src="medalImage" :alt="`Medal level ${medalLevel}`"/>
</template>
<script lang="ts" setup>
    import { ref, onMounted, defineProps, watch } from 'vue';
    import { getSBT } from "@/utils/images";

    const props = defineProps({
        medalLevel: {
            type: Number,
            required: false,
            default: 0,
        },
    });
    const medalImage = ref();

    onMounted(() => {
        getMedalUrl()
    })

    const getMedalUrl = async () => {
        if (props.medalLevel) {
            getSBT(props.medalLevel).then(res => {
                medalImage.value = res;
            });
        }
    }

    watch(
        () => props.medalLevel,
        (nv) => {
            getMedalUrl()
        },
    );

</script>
<style lang="scss" scoped>
    /* 勋章图片样式 */
    img {
        cursor: pointer;
        width: 50px;
    }
</style>
