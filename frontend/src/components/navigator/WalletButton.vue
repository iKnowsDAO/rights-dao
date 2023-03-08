<template>
    <div class="wallet-container">
        <button v-if="!isConnectedWallet" class="connect-button login-wallet" @click="connectWallet()">
            {{t('wallet.wallet')}}
        </button>
        <el-dropdown v-else :hide-timeout="80">
            <button class="connect-button">
                <img src="@/assets/images/dfinityICON.svg">
                <span> {{walletStore.icp}}</span>
            </button>
            <template #dropdown>
                <div class="wallet-status">
                    <div class="title">
                        <h5>{{t('wallet.connected')}}</h5>
                    </div>
                    <div>
                        <el-icon v-if="isBindWallet" style="color: var(--el-color-success);">
                            <SuccessFilled/>
                        </el-icon>
                        {{walletProvider?.wallets[0].principal}}
                        <el-icon class="copy" @click="copy" style="margin-left: 5px">
                            <CopyDocument/>
                        </el-icon>
                    </div>
                    <div>
                        <div v-if="isConnected">
                            <div class="light green">&nbsp;</div>
                            <div>{{t('wallet.online')}}</div>
                        </div>
                        <div v-else>
                            <div class="light red">&nbsp;</div>
                            <div>Offline</div>
                        </div>
                        <el-button type="danger" @click="disconnectWallet()">
                            {{t('wallet.disconnect')}}
                        </el-button>
                        <el-button v-if="isBindWallet" type="warning" @click="unbindWallet()" :loading="bindLoading">
                            {{t('wallet.unbind.button')}}
                        </el-button>
                        <el-button v-else type="primary" @click="bindWallet()" :loading="bindLoading">
                            {{t('wallet.bind.button')}}
                        </el-button>
                    </div>
                </div>
            </template>
        </el-dropdown>
    </div>
</template>
<script lang="ts" setup>
    import { ref, onMounted, watch, defineProps } from 'vue';
    import { ElButton, ElDropdown, ElDropdownItem, ElDropdownMenu, ElIcon } from 'element-plus/es';
    import { CopyDocument, Open, SuccessFilled } from '@element-plus/icons-vue';
    import { ConnectDialog, useDialog, useConnect, useBalance, useWallet } from "@connect2ic/vue"
    import "@connect2ic/core/style.css"
    import { copyUtil } from "@/common/utils";
    import { getTargetUserNewCache, userConnectWallet, userDisConnectWallet } from "@/api/user";
    import { showMessageSuccess } from "@/utils/message";
    import { t } from "@/locale";
    import { deleteUserInfoStorage } from "@/utils/storage";
    import { useWalletStore } from "@/stores/wallet";

    const props = defineProps({
        // 用户已经绑定的钱包principalId
        userWalletPrincipal: {
            type: String,
            required: true,
        },
        userPrincipal: {
            type: String,
            required: true,
        },
    });

    const walletStore = useWalletStore();
    const [walletProvider] = useWallet()
    const {open, close, isOpen} = useDialog()
    const {isConnected, principal, disconnect} = useConnect({
        onConnect: (res) => {
            // Signed in
            isConnectedWallet.value = true;
            console.log("connectWallet onConnect", res)
        },
        onDisconnect: (res) => {
            // Signed out
            console.log("onDisconnect", res)
        }
    })
    const [assets, {refetch, error}] = useBalance()
    const isConnectedWallet = ref(isConnected.value);
    //如果存在钱包principal，则说明已连接钱包
    const isBindWallet = ref(false);
    const bindLoading = ref(false);

    const copy = () => {
        // console.log("connectWallet onConnect", walletProvider.value.wallets)
        //复制
        copyUtil(walletProvider.value.wallets[0].principal);
    }

    const bindWallet = () => {
        bindLoading.value = true;
        //删除store里的用户缓存信息
        if (props.userPrincipal) {
            deleteUserInfoStorage(props.userPrincipal)
        }
        userConnectWallet(walletProvider.value.wallets[0].principal).then(res => {
            if (res.Ok) {
                //刷新用户缓存。
                getTargetUserNewCache(props.userPrincipal)
                showMessageSuccess(t('wallet.bind.success'))
                isBindWallet.value = true;
            }
        }).finally(() => {
            bindLoading.value = false;
        });
    }

    const unbindWallet = () => {
        bindLoading.value = true;
        //删除store里的用户缓存信息
        if (props.userPrincipal) {
            deleteUserInfoStorage(props.userPrincipal)
        }
        userDisConnectWallet().then(res => {
            if (res.Ok) {
                //刷新用户缓存。
                getTargetUserNewCache(props.userPrincipal)
                showMessageSuccess(t('wallet.unbind.success'));
                isBindWallet.value = false;
            }
        }).finally(() => {
            bindLoading.value = false;
        });
    }

    const disconnectWallet = () => {
        //断开钱包链接
        disconnect();
        isConnectedWallet.value = false;
    }

    const connectWallet = () => {
        //点击按钮时如果页面打开，则关闭页面。
        if (isOpen.value) {
            close()
        } else {
            open()
        }
    }

    onMounted(() => {
        refetch();
        //刷新用户缓存。
        getTargetUserNewCache(props.userPrincipal)
        isBindWallet.value = !!props.userWalletPrincipal
    });

    watch(
        () => props.userWalletPrincipal,
        () => {
            isBindWallet.value = !!props.userWalletPrincipal
        })

    watch(
        () => assets,
        () => {
            //将icp数量存入缓存中。
            if (assets) {
                walletStore.icp = assets.value.find((o) => o.symbol === "ICP").amount;
                if (walletProvider) {
                    walletStore.principal = walletProvider.value.wallets[0].principal
                }
            }
        },
        {deep: true})

</script>
<style lang="scss">
    .el-popper {
        .wallet-status {
            padding: 20px 50px;
            font-size: var(--el-font-size-medium);
            color: var(--el-text-color-regular);
            display: flex;
            align-items: center;
            flex-direction: column;
            .title {
                color: black;
                font-weight: 600;
            }
            .light {
                width: 10px;
                height: 10px;
                border-radius: 16px;
                margin-right: 5px;
            }
            .green {
                background: var(--el-color-success);
                box-shadow: 0px 0px 10px 2px var(--el-color-success)
            }
            .red {
                background: var(--el-color-danger);
                box-shadow: 0px 0px 10px 2px var(--el-color-danger)
            }
            .copy {
                cursor: pointer;
            }
            .el-button {
                margin-left: 5px;
            }
            div {
                display: flex;
                align-items: center;
            }
            > div {
                margin-bottom: 10px;
            }
        }
    }
</style>
<style lang="scss" scoped>
    .wallet-container {
        .login-wallet {
            height: 52px;
        }
        .connect-button {
            margin-right: 5px;
            font-size: 18px;
            background: rgb(35 35 39);
            color: white;
            border: none;
            padding: 10px 20px;
            display: flex;
            align-items: center;
            border-radius: 40px;
            cursor: pointer;
            font-weight: 600;
            img {
                margin-right: 5px;
            }
        }
    }
</style>
