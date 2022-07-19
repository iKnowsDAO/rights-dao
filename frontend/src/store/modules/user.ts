import {
    setLocaleStorage,
    getLocaleStorage,
    getUserInfoStorage,
    setUserInfoStorage,
    deleteUserInfoStorage,
} from '@/utils/storage';
import {changeLanguage, findLocaleByString, SupportedLocale} from '../../locale';
import {UserInfo, UserInfoElement} from "@/types/user";

export const getLocaleText = 'getLocale';
export const setLocaleText = 'setLocale';

export const getPrincipalText = 'getPrincipal';
export const setPrincipalText = 'setPrincipal';

export const getUserInfoText = 'getUserInfo';
export const setUserInfoText = 'setUserInfo';
export const setUsernameText = 'setUsername';

export class UserState {
    locale: SupportedLocale | '' = '';
    principal = ''; // 记录当前登录用户的 principal
    user: UserInfo = new UserInfo();
}

const getUserInfoByState = function (state: UserState): UserInfo {
    console.log("getUserInfoByState",state.principal);
    if (!state.principal) return new UserInfo(); // 还没有设置 principal 就都给空
    if (state.user && state.user.owner == state.principal) return state.user;
    // 缓存中没有，就读取
    let readUser = getUserInfoStorage(state.principal);
    if (!readUser) {
        readUser = new UserInfo(); // 如果没有就新建一个空的
        readUser.owner = state.principal;
        setUserInfoStorage(readUser);
    }
    state.user = readUser;
    return readUser;
};

export default {
    namespaced: true,
    state: () => new UserState(),
    mutations: {
        setLocale: (state: UserState, locale: SupportedLocale) => {
            state.locale = locale;
            changeLanguage(locale); // 统一设置语言
            setLocaleStorage(locale); // 持久化 SupportedLocale对象本身就是字符串枚举
        },
        setPrincipal: (state: UserState, principal: string) => {
            if (principal === '') {
                state.user = new UserInfo();
                // 如果是清除用户登录信息 持久化的信息也应该清除
                deleteUserInfoStorage(state.principal);
                state.principal = principal;
            } else if (principal !== state.principal) {
                // 如果是设置新的用户身份 应当重新尝试获取信息
                state.principal = principal;
                getUserInfoByState(state); // 更新对应 principal 的持久化信息
            }
            // 当前 principal 不用持久化
        },
        setUserInfo: (state: UserState, userInfo: UserInfoElement) => {
            state.user = { ...state.user, ...userInfo };
            console.log("setUserInfoState",state)
            setUserInfoStorage(state.user);
        },
        setUsername: (state: UserState, username: string) => {
            const userInfo = getUserInfoByState(state);
            userInfo.name = username;
            setUserInfoStorage(userInfo);
        },
    },
    actions: {
        setLocale: ({ commit }, locale: SupportedLocale) => commit(setLocaleText, locale),
        setPrincipal: ({ commit }, principal: string) => commit(setPrincipalText, principal),
        setUserInfo: ({ commit }, userInfo: UserInfo) => commit(setUserInfoText, userInfo),
        setUsername: ({ commit }, username: string) => commit(setUsernameText, username),
    },
    getters: {
        getLocale: (state: UserState): SupportedLocale => {
            if (state.locale) return state.locale;
            state.locale = findLocaleByString(getLocaleStorage()); // 放入缓存
            return state.locale;
        },
        getUserInfo: getUserInfoByState,
    },
};
