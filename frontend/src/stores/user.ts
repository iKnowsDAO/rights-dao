import { defineStore } from "pinia";
import { changeLanguage, findLocaleByString, SupportedLocale } from "@/locale";
import { UserInfo, UserInfoElement } from "@/types/user";
import {
    deleteUserInfoStorage,
    getLocaleStorage,
    getUserInfoStorage,
    setLocaleStorage,
    setUserInfoStorage
} from "@/utils/storage";

interface UserState {
    locale: SupportedLocale | "";
    principal: string;
    user: UserInfo;
}

//从缓存中读取userInfo
const getUserInfoByState = function (principal: string, user: UserInfo): UserInfo {
    // console.log("getUserInfoByState",state.principal);
    if (!principal) return new UserInfo(); // 还没有设置 principal 就都给空
    if (user && user.owner == principal) return user;
    // 缓存中没有，就读取
    let readUser = getUserInfoStorage(principal);
    if (!readUser) {
        readUser = new UserInfo(); // 如果没有就新建一个空的
        readUser.owner = principal;
        console.log("readUser",readUser)
        setUserInfoStorage(readUser);
    }
    return readUser;
};

export const useUserStore = defineStore({
    id: "user",
    state: (): UserState => ({
        locale: "",
        principal: "",
        user: new UserInfo(),
    }),
    getters: {
        getLocale: (state): SupportedLocale => {
            if (state.locale) return state.locale;
            state.locale = findLocaleByString(getLocaleStorage()); // 放入缓存
            return state.locale;
        },
        getUserInfo: (state): UserInfo => {
            state.user = getUserInfoByState(state.principal, state.user);
            return state.user
        },
    },
    actions: {
        async setLocale(locale: SupportedLocale) {
            this.locale = locale;
            changeLanguage(locale); // 统一设置语言
            setLocaleStorage(locale); // 持久化 SupportedLocale对象本身就是字符串枚举
        },
        async setPrincipal(principal: string) {
            if (principal === '') {
                this.user = new UserInfo();
                // 如果是清除用户登录信息 持久化的信息也应该清除
                deleteUserInfoStorage(this.principal);
                this.principal = principal;
            } else if (principal !== this.principal) {
                // 如果是设置新的用户身份 应当重新尝试获取信息
                this.principal = principal;
                getUserInfoByState(this.principal, this.user); // 更新对应 principal 的持久化信息
            }
            // 当前 principal 不用持久化
        },
        async setUserInfo(userInfo: UserInfoElement) {
            this.user = {...this.user, ...userInfo};
            setUserInfoStorage(this.user);
        },
        async setUsername(username: string) {
            const userInfo = getUserInfoByState(this.principal, this.user);
            userInfo.name = username;
            setUserInfoStorage(userInfo);
        },
    }
});

