<template>
    <div class="bounty-tag" v-if="props.bounty_sum.length>0">
        <img src="@/assets/images/dfinityICON.svg">
        {{SumDivision(props.bounty_sum[0])}}&nbsp;ICP
    </div>
</template>

<script lang="ts" setup>
    import { defineProps } from 'vue';
    import { SumDivision } from "@/api/bounty";

    const props = defineProps({
        // 必要的内容，显示哪些
        bounty_sum: {
            type: Array as () => number[],
            required: true,
        }
    });

</script>

<style lang="scss">
    .bounty-tag {
        display: inline-flex;
        align-items: center;
        padding: 0 5px;
        border-radius: 3px;
        margin-left: 10px;
        img {
            margin-right: 3px;
        }
    }
</style>
