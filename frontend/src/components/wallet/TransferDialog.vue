<template>
    <el-dialog
        v-model="visible"
        :before-close="onClose"
        :title="props.title"
        :width="isPhone? '95%' : '35%'"
        destroy-on-close
        center
    >
    <span style="white-space: pre-line;">
      {{props.content}}
    </span>
        <el-input type="number" v-model="bountyAmount" placeholder="Amount"
                  min="0.0001" step="0.0001"
                  style="margin-bottom: 5px">
            <template #append>ICP</template>
        </el-input>
        <span v-if="!props.onlyTransfer">
            {{t('message.tip.notClose')}}
        </span>
        <br/>
        <span>fee: <s>15%</s>&nbsp 0%</span>
        <el-steps v-if="!props.onlyTransfer" :active="active" finish-status="success" simple style="margin-top: 20px">
            <el-step :title="t('wallet.bounty.createOrder')"/>
            <el-step :title="t('wallet.bounty.transfer')"/>
            <el-step :title="t('wallet.bounty.updateOrder')"/>
        </el-steps>
        <template #footer>
          <span class="dialog-footer">
             <el-button @click="onClose">{{t('common.cancel')}}</el-button>
              <el-button type="primary" @click="submit()" :loading="loading">
                  {{t('common.confirm')}}
              </el-button>
          </span>
        </template>
    </el-dialog>
</template>
<script lang="ts" setup>
    import { ref, onMounted, defineProps, defineEmits } from 'vue';
    import { ElDialog, ElInput, ElButton, ElSteps, ElStep } from 'element-plus/es';
    import { useConnect, useBalance } from "@connect2ic/vue";
    import { t } from '@/locale';
    import { showMessageError, showMessageSuccess } from "@/utils/message";
    import { addPostBounty, updatePostBounty } from "@/api/bounty";
    import { getRandomNumber } from "@/common/utils";

    const {isConnected, activeProvider, principal} = useConnect()
    const [assets, {refetch, error}] = useBalance()

    const props = defineProps({
        visible: {
            type: Boolean,
            required: true,
            default: false
        },
        title: {
            type: String,
            required: true,
        },
        postId: {
            type: Number,
        },
        content: {
            type: String,
            required: true,
        },
        targetPrincipal: {
            type: String,
            required: true,
        },
        onlyTransfer: {
            type: Boolean,
            required: true,
        }
    });
    const loading = ref(false);
    const bountyAmount = ref(0.0001);
    const active = ref(0)
    const random = ref('00000000'); //8位随机数
    const emit = defineEmits(['update:visible'])
    //如果宽度小于769px则说明是平板以下的尺寸
    const isPhone = ref(document.documentElement.clientWidth < 769);

    const check = (value) => {
        value = value.toString()
        //检查‘.’出现的位置
        const index = value.indexOf('.');
        if (index != -1) {
            const str = value.split('.')[1]
            //检查是否小数点后小于8位
            return str.length > 8;
        }
    }

    const submit = async () => {
        const payAmount = bountyAmount.value;
        const timeoutId = setTimeout(() => {
            showMessageError(t('message.error.browser.block'))
        }, 30 * 1000);
        //check
        if (check(bountyAmount.value)) {
            showMessageError(t('message.error.decimalNoMore', {amount: 8}))
            return
        } else if (!(payAmount >= 0.0001)) {
            showMessageError(t('message.error.amountMore', {amount: 0.0001}))
            return;
        }
        loading.value = true;
        //只转账，不做其他的
        if (props.onlyTransfer) {
            const res = await transferToken(props.targetPrincipal, payAmount)
            if (res && res.value) {
                clearTimeout(timeoutId!);
                showMessageSuccess(t('wallet.reward.success'))
                onClose();
            }
            loading.value = false;
            //不做赏金，直接终止方法
            return;
        }
        const payOrderId = await submitPostBounty();
        //保证payOrderId是number，避免ts报错
        if (!payOrderId && typeof payOrderId !== 'number') {
            return
        }
        console.log("transferToken", props.targetPrincipal, payAmount)
        const res = await transferToken(props.targetPrincipal, payAmount)
        if (res && res.value) {
            active.value = 2;
            clearTimeout(timeoutId!);
            //打款成功，更新状态。
            updatePostBounty(payOrderId, payAmount, Number(random.value)).then(res => {
                showMessageSuccess(t('wallet.bounty.success'))
                active.value = 3;
                console.log("updatePostBounty", res)
                onClose();
            }).finally(() => {
                loading.value = false;
            });
        } else {
            loading.value = false;
            console.error("transferToken error", res)
        }
    }

    const submitPostBounty = async () => {
        random.value = getRandomNumber(8)
        console.log("random", random.value);
        const {postId} = props;
        if (!postId) return;
        const res = await addPostBounty(postId, bountyAmount.value, +random.value);
        console.log("addPostBounty res", res);
        if (res.Ok) {
            active.value = 1;
            return Number(res.Ok);
        } else {
            return false;
        }
    }

    const transferToken = async (principalId: string, amount: number) => {
        console.log("isConnected", isConnected.value)
        if (isConnected.value) {
            const result = await activeProvider.value.requestTransfer({
                to: principalId,
                amount: amount,
            })
            refetch();
            if (result.value) {
                showMessageSuccess(t('message.wallet.transfer.success'))
            } else {
                showMessageError(t('message.wallet.transfer.error'))
            }
            return result
        } else {
            console.error("not connect wallet")
            showMessageError(t('message.wallet.connect.error'))
        }
    }

    const onClose = () => {
        emit('update:visible');
    }

</script>
<style lang="scss">
</style>
