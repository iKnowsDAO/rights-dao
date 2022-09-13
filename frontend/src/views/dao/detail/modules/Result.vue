<template>
    <div class="dao-detail-result-container">
        <div class="container">
            <el-card>
                <template #header>
                    <div class="header">
                        <h4><b>{{t('dao.result.title')}}</b></h4>
                    </div>
                </template>
                <div class="dao-result">
                    <div class="percent">
                        <b>{{t('dao.result.yes')}}</b>
                        <span>{{yesPercentage}}%</span>
                    </div>
                    <el-progress :text-inside="true"
                                 :stroke-width="15"
                                 :percentage="yesPercentage"
                                 :show-text="false"
                                 status="success"/>
                    <div class="percent">
                        <b>{{t('dao.result.no')}}</b>
                        <span>{{noPercentage}}%</span>
                    </div>
                    <el-progress :text-inside="true"
                                 :stroke-width="15"
                                 :percentage="noPercentage"
                                 :show-text="false"
                                 status="exception"/>
                </div>
            </el-card>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, defineProps, PropType} from 'vue';
    import {ElCard,ElProgress} from 'element-plus/es';
    import {t} from '@/locale';

    const yesPercentage = ref(0);
    const noPercentage = ref(0);
    const props = defineProps({
        threshold: {
            type: Number,
            required: true,
        },
        yes: {
            type: Number,
            required: true,
        },
        no: {
            type: Number,
            required: true
        }
    });

    const calculatePercentage = (number) => {
        return number / props.threshold;
    }

    const init = () => {
        yesPercentage.value = Number(calculatePercentage(props.yes).toFixed(2));
        noPercentage.value = Number(calculatePercentage(props.no).toFixed(2));
    }

    onMounted(() => {
        console.log("props",props)
        init()
    });

</script>
<style lang="scss">
    .dao-detail-result-container {
        .dao-result{
            .el-progress{
                margin-top: 4px;
                margin-bottom: 14px;
            }
            b{
                /*color: #8590a6;*/
            }
            span{
                float: right;
            }
        }
    }
</style>
