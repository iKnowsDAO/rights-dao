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
                        <el-tooltip>
                            <template #content>
                                <span v-if="calculateThresholdPercentage(yes)>0">
                                    {{t('dao.result.distance',{amount:calculateThresholdPercentage(yes)})}}
                                </span>
                                <span v-else>
                                    {{t('dao.result.complete')}}
                                </span>
                            </template>
                            <span>
                                {{calculatePercentage(yes)}}%
                                <el-icon><QuestionFilled/></el-icon>
                            </span>
                        </el-tooltip>
                    </div>
                    <el-progress :text-inside="true"
                                 :stroke-width="15"
                                 :percentage="calculatePercentage(yes)"
                                 :show-text="false"
                                 status="success"/>
                    <div class="percent">
                        <b>{{t('dao.result.no')}}</b>
                        <el-tooltip>
                            <template #content>
                                <span v-if="calculateThresholdPercentage(no)>0">
                                    {{t('dao.result.distance',{amount:calculateThresholdPercentage(no)})}}
                                </span>
                                <span v-else>
                                    {{t('dao.result.complete')}}
                                </span>
                            </template>
                            <span>
                                {{calculatePercentage(no)}}%
                                <el-icon><QuestionFilled/></el-icon>
                            </span>
                        </el-tooltip>
                    </div>
                    <el-progress :text-inside="true"
                                 :stroke-width="15"
                                 :percentage="calculatePercentage(no)"
                                 :show-text="false"
                                 status="exception"/>
                </div>
            </el-card>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, onMounted, defineProps,computed} from 'vue';
    import {ElCard,ElProgress,ElTooltip,ElIcon} from 'element-plus/es';
    import {QuestionFilled} from '@element-plus/icons-vue';
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

    const calculatePercentage = computed(()=> (number) => {
        //要显示的是百分比，所以扩大100倍
        if(props.yes===0 && props.no===0){
            return 0
        }
        return Number(((number / (props.yes + props.no)) * 100).toFixed(2));
    })
    const calculateThresholdPercentage = computed(()=> (number) => {
        //计算与阈值的百分比
        return  Number(((1 - number / props.threshold) * 100).toFixed(2));
    })

    const init = () => {

    }

    onMounted(() => {
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
