<template>
    <el-dialog
        v-model="visible"
        :before-close="onClose"
        :title="props.title"
        :width="isPhone? '95%' : '35%'"
        destroy-on-close
        center
    >
    <span>
      {{props.content}}
    </span>
        <el-input type="number" v-model="reward" min="0" placeholder="Amount"></el-input>
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
    import { ElDialog, ElInput, ElButton } from 'element-plus/es';
    import { useConnect, useBalance } from "@connect2ic/vue";
    import { t } from '@/locale';
    import { showMessageError, showMessageSuccess } from "@/utils/message";

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
        content: {
            type: String,
            required: true,
        },
        targetPrincipal: {
            type: String,
            required: true,
        }
    });
    const loading = ref(false);
    const reward = ref(0.00);
    const emit = defineEmits(['update:visible'])
    //如果宽度小于769px则说明是平板以下的尺寸
    const isPhone = ref(document.documentElement.clientWidth < 769);

    const submit = () => {
        if (reward.value > 0) {
            loading.value = true;
            transferToken(props.targetPrincipal, reward.value).finally(() => {
                loading.value = false;
            })
        } else {
            showMessageError(t('message.error.amountMore', {amount: 0}))
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
            console.log("transfer    end", result)
            if (result.Ok2) {
                showMessageSuccess(t('message.wallet.transfer.success'))
            }
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
