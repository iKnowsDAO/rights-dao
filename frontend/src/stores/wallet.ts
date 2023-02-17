import { defineStore } from "pinia";
import { deleteWalletInfoStorage, getWalletInfoStorage, setWalletInfoStorage } from "@/utils/storage";

//store.$reset()为清空数据
export const useWalletStore = defineStore({
    id: "wallet",
    state: () => ({
        icp: 0,
        principal: "",
    }),
    getters: {
        getWalletInfo: (state) => {
            state = getWalletInfoStorage();
            return state;
        },
    },
    actions: {
        //正常情况下钱包数据不应该设置为缓存
        setWalletInfo(principal: string, icpAmount: number) {
            if (principal === '') {
                //principal传入为空，则意为清空钱包数据
                this.$state = {
                    icp: 0,
                    principal: "",
                };
                deleteWalletInfoStorage();
            } else {
                const walletInfo = {
                    icp: icpAmount,
                    principal: principal
                };
                setWalletInfoStorage(walletInfo);
                this.$state = walletInfo;
            }
        }

    }
});

